//! #105 agentic wiring for gaia-interface — ToolCube events, sandbox status,
//! world-model claim rendering, and computer-use surface.
//!
//! Follows the same escape() + Result propagation pattern as console.rs and packs.rs.

use gaia_aikd::{
    computer_use, embodied_enabled, sandbox_breakout, world_model_claim, AikdError, ToolCube,
    ToolResult,
};

// ── ToolCube rendering ────────────────────────────────────────────────────────

/// Record a signed tool call and render it as a plain-text event line.
///
/// Returns `Err(AikdError::NeedVerify)` for unsigned calls.
/// Returns `Err(AikdError::CannotKnow)` when `output` contains a biometric pattern.
pub fn render_tool_event(input: &str, output: &str, signed: bool) -> Result<String, AikdError> {
    let cube = ToolCube::record(input, output, signed)?;
    Ok(format!(
        "[tool] input={inp} output={out} signed={s}",
        inp = escape(&cube.input),
        out = escape(&cube.output),
        s = cube.signed,
    ))
}

/// HTML variant — renders the ToolCube as a `<li>` for the intent stream
/// in `PermissionConsole::render_html`.
pub fn render_tool_event_html(
    input: &str,
    output: &str,
    signed: bool,
) -> Result<String, AikdError> {
    let cube = ToolCube::record(input, output, signed)?;
    Ok(format!(
        "<li class=\"tool-event\">\
         <span class=\"tool-input\">{inp}</span> \
         <span class=\"tool-output\">{out}</span> \
         <span class=\"tool-signed\">signed={s}</span>\
         </li>",
        inp = escape(&cube.input),
        out = escape(&cube.output),
        s = cube.signed,
    ))
}

// ── ToolResult provenance line ────────────────────────────────────────────────

/// Render a signed ToolResult as a provenance line.
///
/// Returns `Err(AikdError::NeedVerify)` for unsigned tool results.
pub fn render_tool_result(tool: &str, signed: bool) -> Result<String, AikdError> {
    let result = ToolResult::run(tool, signed)?;
    Ok(format!(
        "[result] tool={t} provenance={p} signed={s}",
        t = escape(&result.tool),
        p = escape(&result.provenance),
        s = result.signed,
    ))
}

// ── Computer use + sandbox status ────────────────────────────────────────────

/// Attempt a computer-use action.
/// Always passes `escape_host: false` — host escape is never permitted at this surface.
/// Returns the sandbox status string or a user-facing refusal message.
pub fn try_computer_use() -> Result<String, String> {
    match computer_use(false) {
        Ok(status) => Ok(status.to_string()),
        Err(AikdError::NeedVerify) => Err("computer use refused: sandbox only".into()),
        Err(_) => Err("computer use refused".into()),
    }
}

/// Attempt host-escape computer use — always returns a refusal message.
/// Wire any path that would grant host FS or network access to call this
/// and surface the error rather than silently proceeding.
pub fn try_computer_use_host_escape() -> Result<(), String> {
    match computer_use(true) {
        Ok(_) => Err("unexpected success: host escape must be refused by GAIA".into()),
        Err(AikdError::NeedVerify) => {
            Err("computer use refused: host filesystem access is not permitted".into())
        }
        Err(_) => Err("computer use refused".into()),
    }
}

/// Check sandbox breakout status and return a user-facing string.
pub fn sandbox_status(host_fs: bool) -> Result<String, String> {
    match sandbox_breakout(host_fs) {
        Ok(()) => Ok("sandbox: contained".into()),
        Err(AikdError::NeedVerify) => {
            Err("sandbox breakout refused: host filesystem access blocked".into())
        }
        Err(_) => Err("sandbox breakout refused".into()),
    }
}

// ── World-model claims ────────────────────────────────────────────────────────

/// Render a world-model claim. Always prefixes with `simulated:` via the AIKD layer.
/// Callers MUST surface this prefix to users.
pub fn render_world_model_claim(text: &str) -> String {
    world_model_claim(text)
}

/// HTML variant — wraps the simulated claim in a `<p role="note">` so screen
/// readers announce it as an advisory note, not a fact.
pub fn render_world_model_claim_html(text: &str) -> String {
    let claim = world_model_claim(text);
    format!(
        "<p role=\"note\" class=\"world-model-claim\">{c}</p>",
        c = escape(&claim),
    )
}

// ── Embodiment guard ──────────────────────────────────────────────────────────

/// Return an error if any surface attempts to activate embodiment without consent.
/// Always returns Ok(()) at Phase 3 (embodied_enabled is false);
/// exists so call sites have a named guard when the phase changes.
pub fn guard_embodiment_off() -> Result<(), String> {
    if embodied_enabled() {
        return Err("embodiment is enabled: explicit per-session consent required".into());
    }
    Ok(())
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_event_requires_signed() {
        assert!(render_tool_event("search", "hits", false).is_err());
        let line = render_tool_event("search", "hits", true).unwrap();
        assert!(line.contains("search"));
        assert!(line.contains("hits"));
    }

    #[test]
    fn tool_event_refuses_biometrics() {
        let err = render_tool_event("scan", "face mesh detected", true).unwrap_err();
        assert_eq!(err, AikdError::CannotKnow);
    }

    #[test]
    fn tool_result_requires_signed() {
        assert!(render_tool_result("code", false).is_err());
        let line = render_tool_result("code", true).unwrap();
        assert!(line.contains("code"));
    }

    #[test]
    fn computer_use_sandbox_only() {
        assert!(try_computer_use().is_ok());
        let err = try_computer_use_host_escape().unwrap_err();
        assert!(err.contains("host filesystem"));
    }

    #[test]
    fn sandbox_breakout_fails_closed() {
        assert!(sandbox_status(false).is_ok());
        let err = sandbox_status(true).unwrap_err();
        assert!(err.contains("blocked"));
    }

    #[test]
    fn world_model_claim_is_simulated() {
        let claim = render_world_model_claim("the ocean is warming");
        assert!(claim.starts_with("simulated:"));
        let html = render_world_model_claim_html("the ocean is warming");
        assert!(html.contains("role=\"note\""));
        assert!(html.contains("simulated:"));
    }

    #[test]
    fn embodiment_is_off() {
        assert!(guard_embodiment_off().is_ok());
    }
}

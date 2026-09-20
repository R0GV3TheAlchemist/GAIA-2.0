//! #102 hallucination wiring for gaia-interface — uncertainty bands, tier
//! floor guards, hallucination warnings, and CannotKnow / NeedVerify surfaces.
//!
//! Follows the same escape() + Result propagation pattern as agentic.rs and packs.rs.

use gaia_aikd::{
    hallucination_warning, propagate_uncertainty, tier_floor, uncertainty_band,
    AikdError, HallucinationClass, HallucinationWarning, UncertaintyBand,
};

// ── Uncertainty band rendering ───────────────────────────────

/// Render the Phase 1 uncertainty band as a plain-text line.
/// Callers MUST NOT present this as a measured or calibrated score.
pub fn render_uncertainty_band() -> String {
    let band = uncertainty_band();
    format!(
        "[uncertainty] lower={l:.2} upper={u:.2} (fixture — not calibrated)",
        l = band.lower,
        u = band.upper,
    )
}

/// HTML variant — wraps the band in a `<p role="note">` so screen readers
/// announce it as advisory, not a factual score.
pub fn render_uncertainty_band_html() -> String {
    let band = uncertainty_band();
    format!(
        "<p role=\"note\" class=\"uncertainty-band\">uncertainty {l:.2}–{u:.2} (fixture)</p>",
        l = band.lower,
        u = band.upper,
    )
}

/// Render the propagated uncertainty band for a given tier.
/// T5 is `NeedVerify` — do not unwrap it into a band line.
pub fn render_propagated_band(tier: u8) -> Result<String, AikdError> {
    let _ = tier_floor(tier)?;
    let band = propagate_uncertainty(tier);
    Ok(format!(
        "[tier-{t}] lower={l:.2} upper={u:.2}",
        t = tier,
        l = band.lower,
        u = band.upper,
    ))
}

// ── Tier floor guard ───────────────────────────────────────

/// Named guard over `tier_floor`. Returns a user-facing error string on refusal.
pub fn guard_tier(tier: u8) -> Result<u8, String> {
    match tier_floor(tier) {
        Ok(t) => Ok(t),
        Err(AikdError::CannotKnow) => Err(format!("tier {tier} is out of range (1–5)")),
        Err(AikdError::NeedVerify) => {
            Err(format!("tier {tier} requires verification before use"))
        }
        Err(_) => Err(format!("tier {tier} refused")),
    }
}

// ── Hallucination warning rendering ─────────────────────────

/// Render a hallucination warning as a plain-text line.
///
/// Returns `Err(AikdError::NeedVerify)` for Fabricated + T5.
pub fn render_hallucination_warning(
    class: HallucinationClass,
    tier: u8,
) -> Result<String, AikdError> {
    let warning = hallucination_warning(class, tier)?;
    Ok(format!(
        "[hallucination] class={c:?} tier={t} msg={m}",
        c = warning.class,
        t = warning.tier,
        m = escape(&warning.message),
    ))
}

/// HTML variant — renders the warning as a `<div role="alert">` for the
/// permission console stream.
pub fn render_hallucination_warning_html(
    class: HallucinationClass,
    tier: u8,
) -> Result<String, AikdError> {
    let warning = hallucination_warning(class, tier)?;
    Ok(format!(
        "<div role=\"alert\" class=\"hallucination-warning tier-{t}\">\
         <span class=\"hclass\">{c:?}</span> \
         <span class=\"hmsg\">{m}</span>\
         </div>",
        t = warning.tier,
        c = warning.class,
        m = escape(&warning.message),
    ))
}

// ── CannotKnow hard-floor surface ────────────────────────────

/// Named hard-floor guard. Returns a user-facing refusal when the question
/// is empty or otherwise triggers `CannotKnow`.
pub fn guard_cannot_know(question: &str) -> Result<(), String> {
    if question.trim().is_empty() {
        return Err("cannot answer: empty question".into());
    }
    Ok(())
}

// ── helpers ──────────────────────────────────────────────

fn escape(value: &str) -> String {
    value
        .replace('&', "&")
        .replace('<', "<")
        .replace('>', ">")
        .replace('"', """)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gaia_aikd::HallucinationClass;

    #[test]
    fn uncertainty_band_is_fixture() {
        let line = render_uncertainty_band();
        assert!(line.contains("0.00"));
        assert!(line.contains("1.00"));
        assert!(line.contains("fixture"));
    }

    #[test]
    fn uncertainty_band_html_has_role_note() {
        let html = render_uncertainty_band_html();
        assert!(html.contains("role=\"note\""));
        assert!(html.contains("fixture"));
    }

    #[test]
    fn propagated_band_t5_needs_verify() {
        assert_eq!(
            render_propagated_band(5).unwrap_err(),
            AikdError::NeedVerify
        );
    }

    #[test]
    fn propagated_band_t3_is_ok() {
        let line = render_propagated_band(3).unwrap();
        assert!(line.contains("tier-3"));
    }

    #[test]
    fn tier_guard_out_of_range() {
        assert!(guard_tier(0).is_err());
        assert!(guard_tier(6).is_err());
    }

    #[test]
    fn tier_guard_t5_needs_verify() {
        let err = guard_tier(5).unwrap_err();
        assert!(err.contains("verification"));
    }

    #[test]
    fn tier_guard_t3_passes() {
        assert_eq!(guard_tier(3).unwrap(), 3);
    }

    #[test]
    fn hallucination_warning_fabricated_t5_needs_verify() {
        assert!(render_hallucination_warning(HallucinationClass::Fabricated, 5).is_err());
    }

    #[test]
    fn hallucination_warning_temporal_t3_ok() {
        let line = render_hallucination_warning(HallucinationClass::Temporal, 3).unwrap();
        assert!(line.contains("Temporal"));
    }

    #[test]
    fn hallucination_warning_html_has_role_alert() {
        let html =
            render_hallucination_warning_html(HallucinationClass::Factual, 4).unwrap();
        assert!(html.contains("role=\"alert\""));
        assert!(html.contains("Factual"));
    }

    #[test]
    fn cannot_know_empty_question() {
        assert!(guard_cannot_know("").is_err());
        assert!(guard_cannot_know("   ").is_err());
        assert!(guard_cannot_know("what is entropy?").is_ok());
    }
}

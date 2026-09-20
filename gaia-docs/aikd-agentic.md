# AIKD agentic first cut (#105)

Working notes. Not AIKD v1.0. No live computer-use product. No host FS.

- Tool sessions live as `gaia_aikd::ToolCube`.
- Every ToolCube requires a signed call. Unsigned calls fail with `AikdError::NeedVerify`.
- Outputs matching biometric patterns (e.g. `face mesh`) are refused with `AikdError::CannotKnow`.
- `ToolResult::run(_, false)` → `NeedVerify`. Tool results are not knowledge until signed.
- `computer_use(escape_host: true)` → `NeedVerify`. Sandbox cannot take the host.
- `computer_use(escape_host: false)` → `"sandboxed"`. Only sandboxed use is permitted.
- All world-model claims are prefixed `simulated:` via `world_model_claim`. Nothing is oracular.
- `sandbox_breakout(host_fs: true)` → `NeedVerify`. Breakout fails closed.
- `embodied_enabled()` → `false`. Embodiment is off by default and optional.
- There is no live computer-use product, no VM backend, and no AIKD v1.0 tag.
- No raw GAIAN biometrics may pass through the multi-agent knowledge bus (see issue #72).

Agentic sessions follow the AIKD Phase 3 ethics: cite-or-refuse, sandbox-or-refuse,
never ambient embodiment, never biometric leakage between agents.

See `gaia-aikd/src/agentic.rs` and `gaia-aikd/src/session.rs` for the implementation.
See `gaia-aikd/tests/session.rs` for the test that locks in all acceptance conditions.

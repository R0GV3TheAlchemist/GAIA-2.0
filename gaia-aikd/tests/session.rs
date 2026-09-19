use gaia_aikd::{embodied_enabled, sandbox_breakout, AikdError, ToolCube};

#[test]
fn signed_session_writes_cube_and_breakout_fails_closed() {
    ToolCube::record("search", "hits", true).unwrap();
    assert_eq!(sandbox_breakout(true).unwrap_err(), AikdError::NeedVerify);
    sandbox_breakout(false).unwrap();
    assert!(!embodied_enabled());
    assert_eq!(
        ToolCube::record("x", "face mesh", true).unwrap_err(),
        AikdError::CannotKnow
    );
}

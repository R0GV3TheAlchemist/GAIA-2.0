//! #541 listed shelf. Existing APIs only.

use gaia_aikd::{
    computer_use, embodied_enabled, sandbox_breakout, world_model_claim, AikdError, ToolResult,
};

#[test]
fn tools_must_be_signed_and_sandboxed() {
    assert_eq!(ToolResult::run("search", false).unwrap_err(), AikdError::NeedVerify);
    assert!(ToolResult::run("search", true).unwrap().signed);
    assert_eq!(computer_use(true).unwrap_err(), AikdError::NeedVerify);
    assert_eq!(computer_use(false).unwrap(), "sandboxed");
}

#[test]
fn world_model_is_simulated_embodied_off() {
    assert!(world_model_claim("rain").starts_with("simulated:"));
    assert_eq!(sandbox_breakout(true).unwrap_err(), AikdError::NeedVerify);
    sandbox_breakout(false).unwrap();
    assert!(!embodied_enabled());
}

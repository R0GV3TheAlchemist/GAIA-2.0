use gaia_aikd::{computer_use, world_model_claim, AikdError, ToolResult};

#[test]
fn sandbox_holds_and_world_model_is_simulated() {
    ToolResult::run("search", true).unwrap();
    assert_eq!(computer_use(true).unwrap_err(), AikdError::NeedVerify);
    assert_eq!(computer_use(false).unwrap(), "sandboxed");
    assert!(world_model_claim("orbit").starts_with("simulated:"));
}

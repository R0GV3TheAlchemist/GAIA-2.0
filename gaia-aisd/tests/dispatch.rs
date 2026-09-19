use gaia_aisd::{aisd_v1_tagged, capability_copy, dispatch, release_gaps, AisdError, Maturity};

#[test]
fn tool_call_needs_skill_id_and_ui_cannot_say_agi() {
    let log = dispatch(Some("aisd:code:review"), Maturity::L3).unwrap();
    assert_eq!(log.skill_id, "aisd:code:review");
    assert_eq!(
        dispatch(None, Maturity::L3).unwrap_err(),
        AisdError::InsufficientMaturity
    );
    assert_eq!(
        capability_copy("autonomous AGI").unwrap_err(),
        AisdError::Unmeasured
    );
    assert!(release_gaps().iter().any(|g| g.contains("no AISD v1.0")));
    assert!(!aisd_v1_tagged());
}

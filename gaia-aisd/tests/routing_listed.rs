//! #487 listed shelf. Existing APIs only.

use gaia_aisd::{
    aisd_v1_tagged, ask_aisd, capability_copy, dispatch, release_gaps, AisdError, Maturity,
};

#[test]
fn dispatch_requires_skill_id_and_logs_maturity() {
    assert_eq!(
        dispatch(None, Maturity::L3).unwrap_err(),
        AisdError::InsufficientMaturity
    );
    let log = dispatch(Some("aisd:science:weather-forecast"), Maturity::L3).unwrap();
    assert_eq!(log.skill_id, "aisd:science:weather-forecast");
    assert_eq!(log.maturity, Maturity::L3);
}

#[test]
fn no_autonomous_agi_copy_and_no_v1() {
    assert_eq!(
        capability_copy("autonomous AGI").unwrap_err(),
        AisdError::Unmeasured
    );
    capability_copy("measured weather skill").unwrap();
    assert!(release_gaps().contains(&"no AISD v1.0"));
    assert_eq!(
        ask_aisd(Maturity::L5, Maturity::L2).unwrap_err(),
        AisdError::InsufficientMaturity
    );
    assert!(!aisd_v1_tagged());
}

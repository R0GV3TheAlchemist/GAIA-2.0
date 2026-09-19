//! #469 listed shelf. Existing APIs only.

use gaia_sos::{discover, evaluate, god_coordinator, live_containerd, live_mcp, second_kernel, SosError};

#[test]
fn intent_discovery_does_not_need_a_roster() {
    assert_eq!(discover(true).unwrap(), "intent-discover-capsule-result");
    assert_eq!(discover(false).unwrap(), "none");
    assert_eq!(god_coordinator().unwrap_err(), SosError::GodCoordinator);
}

#[test]
fn admission_denies_unregistered_and_phase2_is_not_live() {
    let denied = evaluate(false, true, true, true, true);
    assert!(!denied.allowed);
    assert_eq!(denied.reason, "not-in-registry");
    assert!(!live_mcp());
    assert!(!live_containerd());
    assert!(!second_kernel());
}

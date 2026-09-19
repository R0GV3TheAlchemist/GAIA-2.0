//! #465 listed shelf. Existing APIs only.

use gaia_sos::{
    hal_tiers, live_mcp, second_kernel, sentience, sos_v1_tagged, submit_intent, SosError,
    HOST_CALLS,
};

#[test]
fn signed_intent_without_capability_is_refused() {
    assert_eq!(HOST_CALLS.len(), 8);
    assert!(HOST_CALLS.contains(&"intent"));
    assert_eq!(submit_intent(false, true).unwrap_err(), SosError::Unsigned);
    assert_eq!(
        submit_intent(true, false).unwrap_err(),
        SosError::NoCapability
    );
    submit_intent(true, true).unwrap();
}

#[test]
fn no_second_kernel_no_v1_no_live_mcp() {
    assert!(!second_kernel());
    assert!(!sos_v1_tagged());
    assert!(!sentience());
    assert!(!live_mcp());
    assert_eq!(hal_tiers().len(), 5);
}

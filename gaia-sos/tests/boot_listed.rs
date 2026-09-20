//! #527 listed shelf. Existing APIs only.

use gaia_sos::{live_mcp, second_kernel, sos_v1_tagged, threats, HOST_CALLS};

#[test]
fn prompt_injection_is_in_the_threat_model() {
    assert!(threats().contains(&"prompt-injection"));
    assert_eq!(threats().len(), 5);
}

#[test]
fn boot_flags_and_host_surface_stay_local() {
    assert!(!live_mcp());
    assert!(!second_kernel());
    assert!(!sos_v1_tagged());
    assert!(HOST_CALLS.contains(&"intent"));
}

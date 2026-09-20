//! #511 listed shelf. Existing APIs only.

use gaia_sos::{
    five_nines_claimed, live_mcp, refuse_consciousness_runtime, rsi, second_kernel, sentience,
    sos_v1_tagged, submit_intent, HOST_CALLS, SosError,
};

#[test]
fn meta_gates_stay_closed() {
    assert!(!second_kernel());
    assert!(!rsi());
    assert!(!sentience());
    assert!(!five_nines_claimed());
    assert!(!sos_v1_tagged());
    assert!(!live_mcp());
    assert!(refuse_consciousness_runtime().is_err());
}

#[test]
fn unsigned_intent_refused_and_host_calls_exist() {
    assert_eq!(submit_intent(false, true).unwrap_err(), SosError::Unsigned);
    submit_intent(true, true).unwrap();
    assert!(HOST_CALLS.contains(&"intent"));
    assert!(HOST_CALLS.contains(&"learn"));
}

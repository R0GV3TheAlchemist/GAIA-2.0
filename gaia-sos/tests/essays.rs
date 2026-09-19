//! #449 essay bind. Calls APIs already on main.
use gaia_sos::{
    five_nines_claimed, formal_verify_done, rsi, second_kernel, sentience, sos_v1_tagged,
    submit_intent, HOST_CALLS,
};

#[test]
fn essay_sos_does_not_grant_a_kernel() {
    assert!(!second_kernel());
    assert!(!rsi());
    assert!(!sentience());
    assert!(!five_nines_claimed());
    assert!(!sos_v1_tagged());
    assert!(!formal_verify_done());
    assert_eq!(HOST_CALLS.len(), 8);
    assert!(HOST_CALLS.contains(&"intent"));
    assert!(HOST_CALLS.contains(&"learn"));
    assert!(submit_intent(false, true).is_err());
    assert!(submit_intent(true, false).is_err());
    assert!(submit_intent(true, true).is_ok());
}

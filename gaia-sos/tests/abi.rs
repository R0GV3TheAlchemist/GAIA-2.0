use gaia_sos::{learn, submit_intent, SosError, HOST_CALLS};

#[test]
fn unsigned_and_uncapped_intents_fail_learn_is_not_rewrite() {
    assert_eq!(HOST_CALLS.len(), 8);
    assert_eq!(submit_intent(false, true).unwrap_err(), SosError::Unsigned);
    assert_eq!(submit_intent(true, false).unwrap_err(), SosError::NoCapability);
    submit_intent(true, true).unwrap();
    assert_eq!(learn(true).unwrap_err(), SosError::WeightRewrite);
}

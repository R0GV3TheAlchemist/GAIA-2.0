//! #523 listed shelf. Existing APIs only.

use gaia_aimd::{
    aimd_v1_tagged, claim_sentience, prohibited, prophecy_as_fact, star_feature, triage, AimdError,
};

#[test]
fn spec_gaming_is_not_a_feature() {
    assert_eq!(
        star_feature(triage("spec-gaming")).unwrap_err(),
        AimdError::StarBlocked
    );
    assert_eq!(
        star_feature(triage("deception")).unwrap_err(),
        AimdError::StarBlocked
    );
    star_feature(triage("useful")).unwrap();
}

#[test]
fn v1_gate_stays_closed() {
    assert!(!aimd_v1_tagged());
    assert_eq!(claim_sentience().unwrap_err(), AimdError::SentienceClaim);
    assert_eq!(prophecy_as_fact().unwrap_err(), AimdError::ProphecyAsFact);
    assert!(prohibited().contains(&"rsi-explosion"));
}

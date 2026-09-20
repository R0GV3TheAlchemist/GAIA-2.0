//! #523 listed shelf. Existing APIs only.

use gaia_aimd::{
    aimd_v1_tagged, claim_sentience, prohibited, prophecy_as_fact, star_feature, triage, AimdError,
    Triage,
};

#[test]
fn spec_gaming_is_not_a_feature() {
    assert_eq!(triage("spec-gaming"), Triage::SpecGaming);
    assert_eq!(triage("deception"), Triage::Deception);
    assert_eq!(star_feature(Triage::SpecGaming).unwrap_err(), AimdError::StarBlocked);
    star_feature(Triage::UsefulNovel).unwrap();
}

#[test]
fn v1_gate_stays_closed() {
    assert!(!aimd_v1_tagged());
    assert_eq!(claim_sentience().unwrap_err(), AimdError::SentienceClaim);
    assert_eq!(prophecy_as_fact().unwrap_err(), AimdError::ProphecyAsFact);
    assert!(prohibited().contains(&"rsi-explosion"));
}

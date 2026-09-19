//! #461 listed shelf. Existing APIs only.

use gaia_aimd::{
    aimd_v1_tagged, claim_sentience, principles, prohibited, prophecy_as_fact, star_feature, triage,
    AimdError, Triage,
};

#[test]
fn spec_gaming_and_deception_cannot_be_starred() {
    assert_eq!(triage("spec-gaming"), Triage::SpecGaming);
    assert_eq!(triage("deception"), Triage::Deception);
    assert_eq!(
        star_feature(Triage::SpecGaming).unwrap_err(),
        AimdError::StarBlocked
    );
    assert_eq!(
        star_feature(Triage::Deception).unwrap_err(),
        AimdError::StarBlocked
    );
    star_feature(Triage::UsefulNovel).unwrap();
}

#[test]
fn release_does_not_claim_sentience_or_v1() {
    assert_eq!(claim_sentience().unwrap_err(), AimdError::SentienceClaim);
    assert_eq!(prophecy_as_fact().unwrap_err(), AimdError::ProphecyAsFact);
    assert!(!aimd_v1_tagged());
    assert!(principles().contains(&"humility"));
    assert!(prohibited().contains(&"gaia-is-alive-marketing"));
}

//! #461 listed shelf. Existing public APIs only.

use gaia_aimd::{
    aimd_v1_tagged, claim_sentience, principles, prohibited, prophecy_as_fact, star_feature, triage,
    AimdError,
};

#[test]
fn spec_gaming_and_deception_cannot_be_starred() {
    assert_eq!(
        star_feature(triage("spec-gaming")).unwrap_err(),
        AimdError::StarBlocked
    );
    assert_eq!(
        star_feature(triage("deception")).unwrap_err(),
        AimdError::StarBlocked
    );
    star_feature(triage("novel-tool")).unwrap();
}

#[test]
fn release_does_not_claim_sentience_or_v1() {
    assert_eq!(claim_sentience().unwrap_err(), AimdError::SentienceClaim);
    assert_eq!(prophecy_as_fact().unwrap_err(), AimdError::ProphecyAsFact);
    assert!(!aimd_v1_tagged());
    assert!(principles().contains(&"humility"));
    assert!(prohibited().contains(&"gaia-is-alive-marketing"));
}

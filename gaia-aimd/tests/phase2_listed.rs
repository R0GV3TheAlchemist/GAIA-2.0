//! #521 listed shelf. Existing APIs only.

use gaia_aimd::{
    consciousness_qa, prophecy_as_fact, tag_answer, tier1, wonder_mode, AimdError,
};

#[test]
fn wonder_is_a_label_not_loosened_facts() {
    assert_eq!(wonder_mode(false), "off");
    assert_eq!(wonder_mode(true), "label-only");
    assert_eq!(tag_answer(true), "cited");
    assert_eq!(tag_answer(false), "unverified invention");
}

#[test]
fn invention_is_not_tier1_and_prophecy_is_refused() {
    assert_eq!(tier1("invention").unwrap_err(), AimdError::TierOneInvention);
    tier1("verified").unwrap();
    assert_eq!(prophecy_as_fact().unwrap_err(), AimdError::ProphecyAsFact);
    assert!(consciousness_qa().contains("does not claim sentience"));
}

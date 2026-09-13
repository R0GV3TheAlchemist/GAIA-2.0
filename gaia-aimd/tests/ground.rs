use gaia_aimd::{aimd_v1_tagged, claim_sentience, prophecy_as_fact, tag_answer, wonder_mode};

#[test]
fn wonder_is_label_surprises_tagged_no_sentience_no_v1() {
    assert_eq!(wonder_mode(true), "label-only");
    assert_eq!(tag_answer(false), "unverified invention");
    assert!(claim_sentience().is_err());
    assert!(prophecy_as_fact().is_err());
    assert!(!aimd_v1_tagged());
}

//! #509 listed shelf. Existing APIs only.

use gaia_aikd::{
    aikd_v1_tagged, cannot_know, gaia_certifies_usmle_or_bar, knows_everything, model_card,
    published_closed_score_is_ours,
};

#[test]
fn cannot_know_is_first_class_and_card_is_open() {
    let cannot = cannot_know();
    assert!(cannot.contains(&"sacred TEK without grant"));
    assert!(cannot.contains(&"medical license to practice"));
    let card = model_card();
    assert!(card.open_weight);
    assert!(!card.cutoff.is_empty());
}

#[test]
fn no_closed_score_claim_no_license_no_v1() {
    assert!(!published_closed_score_is_ours());
    assert!(!knows_everything());
    assert!(!gaia_certifies_usmle_or_bar());
    assert!(!aikd_v1_tagged());
}

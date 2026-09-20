//! #94 listed shelf on the #96 PR. Existing APIs only.

use gaia_aikd::{
    aikd_v1_tagged, cannot_know, model_card, published_closed_score_is_ours, BenchRow, KnowledgeType,
    AikdError,
};

#[test]
fn fixture_card_is_open_with_cannot_know() {
    let card = model_card();
    assert!(card.open_weight);
    assert_eq!(card.name, "local-llama-fixture");
    assert!(!card.cutoff.is_empty());
    assert!(!card.cannot.is_empty());
    assert_eq!(cannot_know().len(), 4);
    assert_eq!(KnowledgeType::all().len(), 7);
}

#[test]
fn closed_scores_are_not_ours() {
    assert!(!published_closed_score_is_ours());
    assert_eq!(
        BenchRow::claim_closed_as_measured("gpt").unwrap_err(),
        AikdError::ClosedScoreClaim
    );
    let local = BenchRow::local_open();
    assert!(!local.published);
    assert!(!local.gaia_measured);
    assert!(!aikd_v1_tagged());
}

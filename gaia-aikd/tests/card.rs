use gaia_aikd::{cannot_know, model_card, published_closed_score_is_ours};

#[test]
fn local_card_has_cannot_know_and_closed_scores_are_not_ours() {
    let card = model_card();
    assert!(card.open_weight);
    assert!(!card.cannot.is_empty());
    assert!(cannot_know().iter().any(|c| c.contains("license to practice")));
    assert!(!published_closed_score_is_ours());
}

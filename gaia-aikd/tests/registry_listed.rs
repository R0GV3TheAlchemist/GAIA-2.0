//! #457 listed shelf. Existing APIs only.

use gaia_aikd::{
    aikd_v1_tagged, model_card, published_closed_score_is_ours, system_tags, AikdError, BenchRow,
};

#[test]
fn local_fixture_is_the_registered_open_row() {
    let card = model_card();
    assert_eq!(card.name, "local-llama-fixture");
    assert!(card.open_weight);
    let local = BenchRow::local_open();
    assert_eq!(local.model, "local-llama-fixture");
    assert!(!local.published);
    assert!(!local.gaia_measured);
}

#[test]
fn published_gpt_or_claude_cannot_be_gaia_measured() {
    let gpt = BenchRow::closed_reference("gpt-fixture").unwrap();
    assert!(gpt.published);
    assert!(!gpt.gaia_measured);
    assert_eq!(
        BenchRow::claim_closed_as_measured("gpt-4").unwrap_err(),
        AikdError::ClosedScoreClaim
    );
    assert_eq!(
        BenchRow::claim_closed_as_measured("claude-fixture").unwrap_err(),
        AikdError::ClosedScoreClaim
    );
    assert!(!published_closed_score_is_ours());
}

#[test]
fn nineteen_system_tags_and_no_v1() {
    assert_eq!(system_tags().len(), 19);
    assert!(system_tags().contains(&"agentic"));
    assert!(system_tags().contains(&"open-weight"));
    assert!(!aikd_v1_tagged());
}

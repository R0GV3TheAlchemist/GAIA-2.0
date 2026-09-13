use gaia_aikd::{system_tags, AikdError, BenchRow};

#[test]
fn local_model_registered_and_closed_scores_not_measured() {
    let local = BenchRow::local_open();
    assert_eq!(local.model, "local-llama-fixture");
    assert!(!local.gaia_measured);
    let gpt = BenchRow::closed_reference("gpt-fixture").unwrap();
    assert!(gpt.published);
    assert!(!gpt.gaia_measured);
    assert_eq!(
        BenchRow::claim_closed_as_measured("claude-fixture").unwrap_err(),
        AikdError::ClosedScoreClaim
    );
    assert_eq!(system_tags().len(), 19);
}

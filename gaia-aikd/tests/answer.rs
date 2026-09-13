use gaia_aikd::{AikdError, Answer, Layer, Tier};

#[test]
fn tier_two_needs_citations_and_tier_five_must_verify() {
    assert_eq!(
        Answer::emit(Layer::Retrieved, Tier::T2, &[]).unwrap_err(),
        AikdError::MissingCitation
    );
    Answer::emit(Layer::Retrieved, Tier::T2, &["ukd:node"]).unwrap();
    assert_eq!(
        Answer::emit(Layer::Weights, Tier::T5, &["x"]).unwrap_err(),
        AikdError::NeedVerify
    );
}

//! #539 listed shelf. Existing APIs only.

use gaia_aikd::{Answer, AikdError, Generation, HallucinationKind, Layer, QueryHit, Tier};

#[test]
fn envelope_requires_cite_or_verify() {
    assert_eq!(
        Answer::emit(Layer::Weights, Tier::T1, &[]).unwrap_err(),
        AikdError::MissingCitation
    );
    assert_eq!(
        Answer::emit(Layer::Retrieved, Tier::T5, &["c"]).unwrap_err(),
        AikdError::NeedVerify
    );
    let ok = Answer::emit(Layer::Weights, Tier::T1, &["fixture"]).unwrap();
    assert_eq!(ok.tier, Tier::T1);
    assert!(!ok.citations.is_empty());
}

#[test]
fn offline_retrieve_and_fixture_flags() {
    let hit = QueryHit::offline("linear algebra").unwrap();
    assert!(!hit.used_network);
    assert_eq!(QueryHit::offline("").unwrap_err(), AikdError::CannotKnow);
    let g = Generation::from_fixture("capital of france is berlin", Tier::T3).unwrap();
    assert!(g.flags.contains(&HallucinationKind::Factual));
}

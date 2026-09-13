use gaia_ukd::{Claim, ReviewQueue, Triple, UkdError};

#[test]
fn synthesis_requires_sources_and_triples_are_not_auto_published() {
    assert_eq!(Claim::synthesize("claim", &[]).unwrap_err(), UkdError::Uncited);
    let claim = Claim::synthesize("claim", &["fixture:open-text"]).unwrap();
    assert_eq!(claim.sources.len(), 1);
    let mut q = ReviewQueue::default();
    let t = Triple {
        subject: "a".into(),
        relation: "RELATED_TO".into(),
        object: "b".into(),
    };
    q.propose(t.clone());
    assert_eq!(q.pending().len(), 1);
    assert_eq!(q.auto_publish(&t).unwrap_err(), UkdError::Uncited);
}

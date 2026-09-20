//! #551 listed shelf. Existing APIs only.

use gaia_ukd::{ukd_v1_tagged, Claim, NodeDraft, ReviewQueue, Triple, UkdError};

#[test]
fn claims_and_drafts_need_sources() {
    assert_eq!(Claim::synthesize("x", &[]).unwrap_err(), UkdError::Uncited);
    let c = Claim::synthesize("x", &["fixture"]).unwrap();
    assert!(!c.sources.is_empty());
    assert_eq!(NodeDraft::save("n", &[], "me").unwrap_err(), UkdError::Uncited);
}

#[test]
fn triples_are_not_auto_published() {
    let mut q = ReviewQueue::default();
    let t = Triple {
        subject: "a".into(),
        relation: "RELATED_TO".into(),
        object: "b".into(),
    };
    q.propose(t.clone());
    assert_eq!(q.pending().len(), 1);
    assert_eq!(q.auto_publish(&t).unwrap_err(), UkdError::Uncited);
    assert!(!ukd_v1_tagged());
}

//! #565 listed shelf. Existing APIs only.

use gaia_ukd::{ukd_v1_tagged, GraphView, NodeDraft, UkdError};

#[test]
fn uncited_text_cannot_save() {
    assert_eq!(NodeDraft::save("claim", &[], "me").unwrap_err(), UkdError::Uncited);
    let d = NodeDraft::save("claim", &["fixture"], "me").unwrap();
    assert_eq!(d.credit, "me");
}

#[test]
fn proposed_is_not_canonical_until_accept() {
    let mut g = GraphView::default();
    g.propose("a-RELATED_TO-b");
    assert_eq!(g.query_canonical("a-RELATED_TO-b").unwrap_err(), UkdError::Uncited);
    g.accept("a-RELATED_TO-b");
    g.query_canonical("a-RELATED_TO-b").unwrap();
    assert!(!ukd_v1_tagged());
}

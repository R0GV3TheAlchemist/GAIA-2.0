use gaia_ukd::{GraphView, NodeDraft, UkdError};

#[test]
fn uncited_text_cannot_save_and_proposed_is_not_canonical() {
    assert_eq!(
        NodeDraft::save("text", &[], "me").unwrap_err(),
        UkdError::Uncited
    );
    NodeDraft::save("text", &["fixture:source"], "me").unwrap();
    let mut view = GraphView::default();
    view.propose("a-RELATED_TO-b");
    assert_eq!(
        view.query_canonical("a-RELATED_TO-b").unwrap_err(),
        UkdError::Uncited
    );
    view.accept("a-RELATED_TO-b");
    view.query_canonical("a-RELATED_TO-b").unwrap();
}

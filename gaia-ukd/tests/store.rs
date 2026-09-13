use gaia_ukd::{Edge, GraphApi, UkdError};

#[test]
fn realm_query_and_jsonld_sample() {
    let api = GraphApi {
        signed_write: false,
    };
    assert_eq!(api.realm_list().len(), 12);
    assert!(api.jsonld_sample().contains("@id"));
    assert_eq!(
        api.write(&Edge {
            from: "a".into(),
            rel: "RELATED_TO".into(),
            to: "b".into(),
        })
        .unwrap_err(),
        UkdError::Uncited
    );
}

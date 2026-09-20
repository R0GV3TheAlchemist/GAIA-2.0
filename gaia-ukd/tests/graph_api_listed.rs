//! #555 listed shelf. Existing APIs only.

use gaia_ukd::{ukd_v1_tagged, Edge, GraphApi, UkdError};

#[test]
fn realm_list_and_unsigned_write() {
    let api = GraphApi::default();
    assert_eq!(api.realm_list().len(), 12);
    let e = Edge {
        from: "a".into(),
        rel: "RELATED_TO".into(),
        to: "b".into(),
    };
    assert_eq!(api.write(&e).unwrap_err(), UkdError::Uncited);
    let signed = GraphApi {
        signed_write: true,
        ..GraphApi::default()
    };
    signed.write(&e).unwrap();
}

#[test]
fn jsonld_sample_is_a_fixture() {
    let s = GraphApi::default().jsonld_sample();
    assert!(s.contains("@id"));
    assert!(s.contains("license"));
    assert!(!ukd_v1_tagged());
}

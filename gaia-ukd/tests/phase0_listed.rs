//! #545 listed shelf. Existing APIs only.

use gaia_ukd::{federated_wikidata, get_node, list_realms, ukd_v1_tagged, UkdError};

#[test]
fn twelve_realms_and_seeded_node() {
    assert_eq!(list_realms().len(), 12);
    assert!(get_node("ukd:math:linear-algebra").is_ok());
    assert_eq!(get_node("ukd:nope").unwrap_err(), UkdError::UnknownNode);
}

#[test]
fn federated_query_cannot_write_tek() {
    assert_eq!(federated_wikidata(true).unwrap_err(), UkdError::TekSealed);
    assert!(!ukd_v1_tagged());
}

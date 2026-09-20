//! #507 listed shelf. Existing APIs only.

use gaia_ukd::{
    federated_wikidata, get_node, tek_export, ukd_v1_tagged, REALMS, UkdError,
};

#[test]
fn twelve_realms_and_cited_ukd_ids() {
    assert_eq!(REALMS.len(), 12);
    assert!(REALMS.contains(&"traditional-knowledge"));
    let n = get_node("ukd:math:linear-algebra").unwrap();
    assert!(n.id.starts_with("ukd:"));
    assert!(!n.citation.is_empty());
    assert!(get_node("skill:active-listening").is_err());
}

#[test]
fn tek_and_v1_stay_gated() {
    assert_eq!(federated_wikidata(true).unwrap_err(), UkdError::TekSealed);
    assert!(federated_wikidata(false).unwrap().contains("not a live"));
    assert_eq!(tek_export(false).unwrap_err(), UkdError::NoAgreement);
    assert!(!ukd_v1_tagged());
}

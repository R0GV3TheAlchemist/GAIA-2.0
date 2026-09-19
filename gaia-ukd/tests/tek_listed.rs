//! #473 listed shelf. Existing APIs only.

use gaia_ukd::{
    publish_tek, tek_export, ukd_v1_tagged, CollectionState, TekGraph, TekStore, UkdError,
};

#[test]
fn default_tek_graph_is_empty_and_sealed() {
    assert!(TekStore::new().is_empty());
    let graph = TekGraph::new();
    assert_eq!(graph.state, CollectionState::Sealed);
    assert_eq!(graph.list_public().unwrap_err(), UkdError::TekSealed);
}

#[test]
fn no_agreement_no_bulk_import_and_withdraw_clears_fixture() {
    assert_eq!(publish_tek(false, "story").unwrap_err(), UkdError::NoAgreement);
    assert_eq!(tek_export(false).unwrap_err(), UkdError::NoAgreement);
    let mut graph = TekGraph::new();
    assert_eq!(graph.ingest_wipo().unwrap_err(), UkdError::NoAgreement);
    graph.load_fixture("fixture-row");
    graph.withdraw();
    assert_eq!(graph.state, CollectionState::Sealed);
    assert_eq!(graph.list_public().unwrap_err(), UkdError::TekSealed);
    assert!(!ukd_v1_tagged());
}

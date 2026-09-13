use gaia_ukd::{CollectionState, TekGraph, UkdError};

#[test]
fn tek_default_is_sealed_and_withdrawal_clears_replicas() {
    let mut graph = TekGraph::new();
    assert_eq!(graph.state, CollectionState::Sealed);
    assert_eq!(graph.list_public().unwrap_err(), UkdError::TekSealed);
    assert_eq!(graph.ingest_wipo().unwrap_err(), UkdError::NoAgreement);
    graph.load_fixture("secret-fixture");
    graph.withdraw();
    assert!(graph.list_public().is_err());
}

use gaia_ukd::{federated_wikidata, get_node, list_realms, UkdError};

#[test]
fn client_can_list_realms_and_fetch_a_node() {
    assert_eq!(list_realms().len(), 12);
    let node = get_node("ukd:eng:quantum-computing").unwrap();
    assert_eq!(node.realm, "engineering");
    assert!(!node.citation.is_empty());
    assert_eq!(federated_wikidata(true).unwrap_err(), UkdError::TekSealed);
    assert!(federated_wikidata(false).unwrap().contains("wikidata-stub"));
}

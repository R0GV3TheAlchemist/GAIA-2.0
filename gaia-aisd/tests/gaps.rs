use gaia_aisd::{gap_nodes, realm_stubs, REALMS, REALM_COUNT};

#[test]
fn thirteen_realms_and_gaps_are_queryable() {
    assert_eq!(REALMS.len(), REALM_COUNT);
    assert_eq!(realm_stubs().len(), 13);
    assert!(gap_nodes().contains(&"swe-pro-engineering"));
}

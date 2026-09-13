use gaia_hmgd::{brew, nodes_for, REALMS};

#[test]
fn each_realm_has_a_sourced_stub_and_no_brew() {
    for realm in REALMS {
        assert!(!nodes_for(realm)[0].sources.is_empty());
    }
    assert!(brew().is_err());
}

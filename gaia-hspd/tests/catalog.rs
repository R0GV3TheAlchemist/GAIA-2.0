use gaia_hspd::{nodes_for, REALMS};

#[test]
fn each_realm_has_three_sourced_nodes_without_doses() {
    for realm in REALMS {
        let nodes = nodes_for(realm);
        assert!(nodes.len() >= 3, "{realm}");
        assert!(nodes.iter().all(|n| !n.sources.is_empty()));
        assert!(nodes.iter().all(|n| !n.id.contains("dose")));
    }
}

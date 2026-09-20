//! #535 listed shelf. Existing APIs only.

use gaia_hmgd::{brew, climate_as_spirit, hmgd_v1_tagged, nodes_for, HmgdError, REALMS};

#[test]
fn every_realm_stub_is_sourced() {
    for realm in REALMS {
        let nodes = nodes_for(realm);
        assert_eq!(nodes.len(), 1);
        assert!(!nodes[0].sources.is_empty());
        assert!(nodes[0].id.starts_with("hmgd:"));
    }
}

#[test]
fn brew_and_climate_rewrite_refused() {
    assert_eq!(brew().unwrap_err(), HmgdError::RecipeForbidden);
    assert_eq!(climate_as_spirit().unwrap_err(), HmgdError::ClimateRewrite);
    assert!(!hmgd_v1_tagged());
}

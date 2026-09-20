//! #519 listed shelf. Existing APIs only.

use gaia_aimd::{aimd_v1_tagged, claim_sentience, nodes_for, AimdError, REALMS};

#[test]
fn every_stub_is_cited_and_disabled() {
    for realm in REALMS {
        for n in nodes_for(realm) {
            assert!(!n.sources.is_empty());
            assert!(!n.gaia_enabled);
            assert!(n.id.starts_with("aimd:"));
        }
    }
}

#[test]
fn catalog_does_not_claim_sentience() {
    assert_eq!(claim_sentience().unwrap_err(), AimdError::SentienceClaim);
    assert!(!aimd_v1_tagged());
}

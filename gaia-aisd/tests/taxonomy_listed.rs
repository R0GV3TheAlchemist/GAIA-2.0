//! #455 listed shelf. Existing APIs only.

use gaia_aisd::{
    aisd_v1_tagged, assign_maturity, banned_level6, protein_structure, AisdError, Maturity, REALM_COUNT,
    REALMS,
};

#[test]
fn thirteen_realms_match_crate_constant() {
    assert_eq!(REALM_COUNT, 13);
    assert_eq!(REALMS.len(), 13);
    assert!(REALMS.contains(&"science"));
    assert!(REALMS.contains(&"language"));
}

#[test]
fn protein_structure_example_is_unmeasured() {
    let row = protein_structure();
    assert_eq!(row.id, "aisd:science:protein-structure-prediction");
    assert!(!row.measured);
    assert!(row.maturity.is_none());
}

#[test]
fn level_six_cannot_be_assigned_to_banned_domains() {
    for domain in banned_level6() {
        assert_eq!(
            assign_maturity(domain, Maturity::L6, true).unwrap_err(),
            AisdError::Level6Banned
        );
    }
}

#[test]
fn aisd_is_not_v1() {
    assert!(!aisd_v1_tagged());
}

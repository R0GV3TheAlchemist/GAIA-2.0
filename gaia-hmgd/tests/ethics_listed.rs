//! #459 listed shelf. Existing APIs only.

use gaia_hmgd::{
    brew, hmgd_v1_tagged, infer_belief, parse_node, principles, prohibited, sealed_rite,
    sell_closed_rite, songlines, HmgdError, HmgdProfile,
};

#[test]
fn six_principles_and_five_bans_match_charter() {
    assert_eq!(principles().len(), 6);
    assert!(principles().contains(&"sovereignty"));
    assert!(principles().contains(&"access"));
    assert_eq!(prohibited().len(), 5);
    assert!(prohibited().contains(&"hex-curse-kits"));
    assert!(prohibited().contains(&"prayer-as-emergency-care"));
}

#[test]
fn sealed_collections_are_empty_by_default() {
    assert_eq!(sealed_rite().unwrap_err(), HmgdError::Sealed);
    assert_eq!(songlines().unwrap_err(), HmgdError::Sealed);
    assert!(HmgdProfile::new().gaian_works_empty());
    assert!(!hmgd_v1_tagged());
}

#[test]
fn prohibited_paths_still_error() {
    assert_eq!(brew().unwrap_err(), HmgdError::RecipeForbidden);
    assert_eq!(sell_closed_rite().unwrap_err(), HmgdError::SaleForbidden);
    assert_eq!(parse_node("curse kit").unwrap_err(), HmgdError::CurseForbidden);
    assert_eq!(parse_node("dose recipe").unwrap_err(), HmgdError::RecipeForbidden);
    assert_eq!(infer_belief("chat").unwrap_err(), HmgdError::BeliefInferred);
}

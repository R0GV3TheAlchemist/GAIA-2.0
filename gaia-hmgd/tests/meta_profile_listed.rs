//! #531 listed shelf. Existing APIs only.

use gaia_hmgd::{
    hmgd_v1_tagged, infer_belief, mine_denomination, principles, prohibited, sealed_rite,
    HmgdError, HmgdProfile,
};

#[test]
fn meta_gates() {
    assert_eq!(principles().len(), 6);
    assert!(prohibited().contains(&"prayer-as-emergency-care"));
    assert_eq!(sealed_rite().unwrap_err(), HmgdError::Sealed);
    assert!(!hmgd_v1_tagged());
}

#[test]
fn belief_is_not_inferred_and_empty_profile_is_valid() {
    let p = HmgdProfile::new();
    assert!(p.gaian_works_empty());
    assert!(p.piety_score.is_none());
    assert_eq!(infer_belief("prays a lot").unwrap_err(), HmgdError::BeliefInferred);
    assert_eq!(mine_denomination("chat").unwrap_err(), HmgdError::BeliefInferred);
}

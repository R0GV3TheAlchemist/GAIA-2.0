use gaia_hspd::{HspdError, HspdProfile};

#[test]
fn child_cannot_have_genetic_profile_and_dna_is_not_required() {
    let profile = HspdProfile::new();
    assert!(profile.genetic_indicators.is_empty());
    assert!(!profile.dna_required);
    assert_eq!(
        HspdProfile::child_genetic(12).unwrap_err(),
        HspdError::ChildGenetic
    );
}

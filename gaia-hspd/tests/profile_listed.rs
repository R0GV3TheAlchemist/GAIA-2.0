//! #463 listed shelf. Existing APIs only.

use gaia_hspd::{hspd_v1_tagged, checkout, HspdError, HspdProfile};

#[test]
fn vault_profile_declares_and_does_not_infer_dna() {
    let p = HspdProfile::new();
    assert!(p.genetic_indicators.is_empty());
    assert!(!p.dna_required);
    assert!(p.declared.is_empty());
    assert_eq!(
        HspdProfile::infer_actn3("raw-file").unwrap_err(),
        HspdError::ChildGenetic
    );
}

#[test]
fn under_sixteen_cannot_add_surgical_or_pharmacologic_tags() {
    assert_eq!(
        HspdProfile::child_tag(15, "surgical").unwrap_err(),
        HspdError::ChildTag
    );
    assert_eq!(
        HspdProfile::child_tag(15, "pharmacologic").unwrap_err(),
        HspdError::ChildTag
    );
    HspdProfile::child_tag(15, "practice").unwrap();
    assert_eq!(
        HspdProfile::child_genetic(15).unwrap_err(),
        HspdError::ChildGenetic
    );
}

#[test]
fn not_a_clinic_and_not_v1() {
    assert_eq!(checkout("kit").unwrap_err(), HspdError::CheckoutForbidden);
    assert!(!hspd_v1_tagged());
}

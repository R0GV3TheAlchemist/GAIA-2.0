//! #525 listed shelf. Existing APIs only.

use gaia_aikd::{gaia_certifies_usmle_or_bar, Adapter, AikdError, Executed, Tier};

#[test]
fn adapters_publish_eval_slice_names_not_licenses() {
    for a in [
        Adapter::Math,
        Adapter::Code,
        Adapter::Science,
        Adapter::Vision,
        Adapter::MedicalRef,
        Adapter::LegalRef,
    ] {
        assert!(!a.eval_slice().is_empty());
        assert!(!a.is_practice_license());
    }
    assert_eq!(
        Adapter::MedicalRef.answer(false, true).unwrap_err(),
        AikdError::MissingCitation
    );
    assert!(Adapter::LegalRef.answer(true, true).unwrap().contains("not legal"));
    assert!(!gaia_certifies_usmle_or_bar());
}

#[test]
fn failed_execution_is_not_tier1() {
    let fail = Executed::code(false, Some("boom"));
    assert_eq!(fail.tier, Tier::T4);
    assert!(fail.error.is_some());
    let proof = Executed::proof(false);
    assert_eq!(proof.tier, Tier::T4);
    let pass = Executed::code(true, None);
    assert_eq!(pass.tier, Tier::T1);
}

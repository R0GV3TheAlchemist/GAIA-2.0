use gaia_aikd::{Adapter, AikdError};

#[test]
fn professional_packs_need_cite_and_are_not_licenses() {
    assert!(!Adapter::MedicalRef.is_practice_license());
    assert_eq!(
        Adapter::MedicalRef.answer(false, true).unwrap_err(),
        AikdError::MissingCitation
    );
    assert!(Adapter::MedicalRef.answer(true, true).unwrap().contains("not medical"));
    assert!(!Adapter::Math.eval_slice().is_empty());
}

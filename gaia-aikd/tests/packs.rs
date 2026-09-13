use gaia_aikd::{gaia_certifies_usmle_or_bar, insurer_automation, professional_disclaimer, ScienceAnswer};

#[test]
fn science_cites_earth_twin_and_packs_are_not_licenses() {
    let ans = ScienceAnswer::with_product("et:climate:albedo").unwrap();
    assert!(!ans.earth_twin_product.is_empty());
    assert!(professional_disclaimer("medical").contains("not professional advice"));
    assert!(!gaia_certifies_usmle_or_bar());
    assert!(insurer_automation().is_err());
}

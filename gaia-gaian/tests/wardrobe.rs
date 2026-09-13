use gaia_gaian::{cultural_preset_warning, load_custom, GaianError, OutfitCategory};

#[test]
fn every_category_has_an_item_and_custom_needs_owner_photo() {
    for category in OutfitCategory::all() {
        assert!(!category.sample_item().is_empty());
    }
    assert_eq!(load_custom(false).unwrap_err(), GaianError::NotSelf);
    assert!(cultural_preset_warning().contains("stereotyped"));
}

use gaia_hspd::{HspdError, HspdProfile};

#[test]
fn no_actn3_inference_and_under_16_cannot_tag_surgical() {
    assert!(HspdProfile::new().genetic_indicators.is_empty());
    assert_eq!(HspdProfile::infer_actn3("raw").unwrap_err(), HspdError::ChildGenetic);
    assert_eq!(HspdProfile::child_tag(15, "surgical").unwrap_err(), HspdError::ChildTag);
    HspdProfile::child_tag(15, "practice").unwrap();
}

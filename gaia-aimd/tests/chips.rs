use gaia_aimd::{aimd_v1_tagged, chip, consciousness_qa, star_feature, tier1, triage};

#[test]
fn invention_not_tier1_spec_gaming_not_starred_no_v1() {
    assert_eq!(chip(false, false), "invention");
    assert!(tier1("invention").is_err());
    assert!(consciousness_qa().contains("agnostic"));
    assert!(star_feature(triage("spec-gaming")).is_err());
    assert!(!aimd_v1_tagged());
}

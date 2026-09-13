use gaia_hmgd::{hmgd_v1_tagged, infer_belief, sealed_rite, songlines, HmgdProfile};

#[test]
fn belief_not_inferred_tek_sealed_no_v1() {
    assert!(HmgdProfile::new().piety_score.is_none());
    assert!(infer_belief("prays a lot").is_err());
    assert!(sealed_rite().is_err());
    assert!(songlines().is_err());
    assert!(!hmgd_v1_tagged());
}

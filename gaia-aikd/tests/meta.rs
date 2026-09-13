use gaia_aikd::{aikd_v1_tagged, audit_report, knows_everything};

#[test]
fn release_does_not_claim_omniscience_or_v1() {
    assert!(!knows_everything());
    assert!(!aikd_v1_tagged());
    assert!(audit_report().iter().any(|l| l.contains("cannot-know")));
}

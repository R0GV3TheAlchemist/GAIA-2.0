use gaia_aispd::{aispd_v1_tagged, principles, prohibited};

#[test]
fn charter_and_no_v1() {
    assert_eq!(principles().len(), 6);
    assert!(prohibited()
        .iter()
        .any(|p| p.contains("intelligence-explosion")));
    assert!(!aispd_v1_tagged());
}

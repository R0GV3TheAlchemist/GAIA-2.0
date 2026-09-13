use gaia_si::{actuate, articles, cameras_default, kill, Stream, Twin};

#[test]
fn charter_no_cameras_no_pii_unauthorized_denied() {
    assert_eq!(articles().len(), 8);
    assert!(!cameras_default());
    let s = Stream::plant("hvac");
    assert!(s.pii.is_empty());
    assert!(!s.cameras);
    assert!(Twin::building().household_id.is_none());
    assert!(Twin::building().uncertainty);
    assert!(actuate(false).is_err());
    assert!(kill());
}

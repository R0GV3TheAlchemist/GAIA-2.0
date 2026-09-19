//! #449 essay bind. Calls APIs already on main.
use gaia_si::{
    actuate, admit, advise, articles, cameras_default, face_field, pathogen_protocol, si_v1_tagged,
    Stream, Twin,
};

#[test]
fn essay_si_is_language_not_v1() {
    assert!(!si_v1_tagged());
    assert_eq!(articles().len(), 8);
    assert!(!cameras_default());
    assert!(face_field().is_err());
    assert!(pathogen_protocol().is_err());
    assert!(admit("").is_err());
    let s = Stream::plant("structural-health");
    assert!(s.pii.is_empty());
    assert!(!s.cameras);
    assert_eq!(s.autonomy, "none");
    assert!(Twin::building().household_id.is_none());
    assert!(advise("conscious building", false).is_err());
    assert!(advise("grid flip", false).is_err());
    assert_eq!(advise("report leak", false).unwrap(), "advisory");
    assert!(actuate(false).is_err());
}

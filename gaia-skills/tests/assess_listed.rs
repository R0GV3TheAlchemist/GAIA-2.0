//! #499 listed shelf. Existing APIs only.

use gaia_skills::{hidden_profile_api, Badge, Session, SkillError};

#[test]
fn owner_started_session_stops_sensors() {
    assert_eq!(Session::start(false).unwrap_err(), SkillError::AmbientDenied);
    let mut s = Session::start(true).unwrap();
    assert!(s.camera && s.mic);
    s.end();
    assert!(!s.camera && !s.mic);
}

#[test]
fn badge_is_presented_not_a_hidden_profile() {
    assert_eq!(
        Badge::mint("skill:suture", true).unwrap_err(),
        SkillError::ClinicalCert
    );
    let b = Badge::mint("skill:active-listening", false).unwrap();
    assert_eq!(b.skill_id, "skill:active-listening");
    assert!(!b.level.is_empty());
    assert!(!b.issuer.is_empty());
    assert!(!b.owner_key.is_empty());
    assert!(!hidden_profile_api());
}

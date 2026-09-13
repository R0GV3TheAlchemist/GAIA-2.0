use gaia_skills::{hidden_profile_api, Badge, Session, SkillError};

#[test]
fn session_is_owner_started_and_sensors_stop() {
    assert_eq!(Session::start(false).unwrap_err(), SkillError::AmbientDenied);
    let mut session = Session::start(true).unwrap();
    session.record("clip");
    session.end();
    assert!(!session.camera && !session.mic);
    session.withdraw();
    assert_eq!(Badge::mint("skill:suture", true).unwrap_err(), SkillError::ClinicalCert);
    let badge = Badge::mint("skill:active-listening", false).unwrap();
    assert!(!badge.owner_key.is_empty());
    assert!(!hidden_profile_api());
}

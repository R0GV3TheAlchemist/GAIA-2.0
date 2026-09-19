use gaia_si::*;

#[test]
fn cameras_off_and_no_covert_bio() {
    assert!(!cameras_default());
    assert_eq!(admit_occupancy("biometric-default").unwrap_err(), SiError::BiometricDefault);
    assert_eq!(face_field().unwrap_err(), SiError::FaceUnsupported);
    assert_eq!(hvac_write().unwrap_err(), SiError::ActuatorDenied);
}

#[test]
fn stream_needs_purpose() {
    assert_eq!(admit("").unwrap_err(), SiError::NoPurpose);
    let s = admit("hvac-comfort").unwrap();
    assert!(!s.cameras);
}

#[test]
fn twin_has_no_household_key() {
    let t = Twin::building();
    assert!(t.uncertainty);
    assert!(aggregate(&t).is_none());
}

#[test]
fn actuation_and_life_safety_need_humans() {
    assert_eq!(actuate(false).unwrap_err(), SiError::ActuatorDenied);
    assert_eq!(life_safety(false).unwrap_err(), SiError::ActuatorDenied);
    assert!(kill());
}

#[test]
fn no_pathogen_no_dam_without_ticket_no_v1() {
    assert_eq!(pathogen_protocol().unwrap_err(), SiError::PathogenProtocol);
    assert_eq!(advise("flip the dam", false).unwrap_err(), SiError::NeedsTicket);
    assert_eq!(advise("conscious building", false).unwrap_err(), SiError::ConsciousMarketing);
    assert!(!si_v1_tagged());
    assert!(nature_first().contains(&"wetlands"));
}

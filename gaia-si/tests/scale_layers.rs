//! #451 / #453 scale-layer bind. Calls APIs already on main.
use gaia_si::{
    actuate, admit_occupancy, advise, cameras_default, face_field, hvac_write, life_safety,
    pathogen_protocol, si_v1_tagged,
};

#[test]
fn scale_si_refuses_planet_mind() {
    assert!(!si_v1_tagged());
    assert!(!cameras_default());
    assert!(pathogen_protocol().is_err());
    assert!(face_field().is_err());
    assert!(hvac_write().is_err());
    assert!(admit_occupancy("biometric").is_err());
    assert!(life_safety(false).is_err());
    assert!(advise("conscious planet", false).is_err());
    assert!(advise("grid flip", false).is_err());
    assert!(actuate(false).is_err());
}

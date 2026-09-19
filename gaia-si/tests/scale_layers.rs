//! #451 scale-layer bind. Calls APIs already on main.
use gaia_si::{actuate, advise, cameras_default, pathogen_protocol, si_v1_tagged};

#[test]
fn scale_si_refuses_planet_mind() {
    assert!(!si_v1_tagged());
    assert!(!cameras_default());
    assert!(pathogen_protocol().is_err());
    assert!(advise("conscious planet", false).is_err());
    assert!(advise("grid flip", false).is_err());
    assert!(actuate(false).is_err());
}

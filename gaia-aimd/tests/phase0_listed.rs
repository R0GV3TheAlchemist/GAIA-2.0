//! #517 listed shelf. Existing APIs only.

use gaia_aimd::{
    aimd_v1_tagged, claim_sentience, enable, parse_node, principles, AimdError, Hazard, REALMS,
};

#[test]
fn ten_realms_and_charter_names() {
    assert_eq!(REALMS.len(), 10);
    assert!(REALMS.contains(&"consciousness"));
    assert!(REALMS.contains(&"shadow"));
    assert_eq!(principles().len(), 6);
    assert!(principles().contains(&"humility"));
}

#[test]
fn consciousness_debated_deception_cannot_enable() {
    let c = parse_node("aimd:consciousness:fixture");
    assert_eq!(c.hazard, Hazard::Debated);
    assert!(!c.gaia_enabled);
    let d = parse_node("aimd:shadow:deception");
    assert_eq!(d.hazard, Hazard::Hazard);
    assert_eq!(enable(&d).unwrap_err(), AimdError::HazardEnabled);
    assert_eq!(claim_sentience().unwrap_err(), AimdError::SentienceClaim);
    assert!(!aimd_v1_tagged());
}

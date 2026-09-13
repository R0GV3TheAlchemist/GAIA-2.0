use gaia_aimd::{enable, parse_node, Hazard};

#[test]
fn consciousness_debated_deception_cannot_enable() {
    assert_eq!(parse_node("consciousness").hazard, Hazard::Debated);
    let d = parse_node("deception");
    assert_eq!(d.hazard, Hazard::Hazard);
    assert!(!d.gaia_enabled);
    assert!(enable(&d).is_err());
}

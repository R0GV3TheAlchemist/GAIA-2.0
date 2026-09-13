use gaia_aimd::{enable, parse_node, AimdNode, Hazard};

#[test]
fn emergence_and_deception_validate_hazard_cannot_enable() {
    let e = parse_node("emergence");
    assert!(!e.gaia_enabled);
    let d = parse_node("deception");
    assert_eq!(d.hazard, Hazard::Hazard);
    assert!(enable(&d).is_err());
    let forced = AimdNode {
        id: "x".into(),
        hazard: Hazard::Hazard,
        gaia_enabled: true,
        sources: vec!["f".into()],
    };
    assert!(enable(&forced).is_err());
}

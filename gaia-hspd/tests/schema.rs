use gaia_hspd::{parse_node, HspdError, RiskClass};

#[test]
fn flow_and_rapamycin_validate_and_dose_is_rejected() {
    let flow = parse_node("flow-state").unwrap();
    assert_eq!(flow.risk, RiskClass::Practice);
    let rapa = parse_node("rapamycin literature").unwrap();
    assert_eq!(rapa.risk, RiskClass::ResearchOnly);
    assert_eq!(
        parse_node("{\"dose\":\"x\"}").unwrap_err(),
        HspdError::DoseForbidden
    );
    assert!(rapa.attach_diy_path().is_err());
}

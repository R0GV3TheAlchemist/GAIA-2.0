use gaia_aispd::{close_finding, label_trace, AispdError, Finding};

#[test]
fn traces_label_and_deception_is_not_wontfix() {
    assert_eq!(label_trace("tool-use surprise"), Finding::EmergentUnexpected);
    assert_eq!(label_trace("deception"), Finding::Deception);
    assert_eq!(close_finding(true).unwrap_err(), AispdError::WontfixBlocked);
}

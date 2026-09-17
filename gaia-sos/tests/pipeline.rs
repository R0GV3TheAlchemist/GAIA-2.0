use gaia_sos::{evaluate, payload_ok};

#[test]
fn default_deny_without_consent_or_registry() {
    let no_reg = evaluate(false, true, true, true, true);
    assert!(!no_reg.allowed);
    let no_consent = evaluate(true, false, true, true, true);
    assert_eq!(no_consent.reason, "consent-missing");
}

#[test]
fn whitelist_and_guardrails_gate_execute() {
    assert!(!payload_ok("Bad Key", "x", 10));
    assert!(payload_ok("goal", "research", 500));
    let ok = evaluate(true, true, true, true, true);
    assert!(ok.allowed);
    assert_eq!(ok.reason, "execute-local");
    let blocked = evaluate(true, true, true, false, true);
    assert!(!blocked.allowed);
}

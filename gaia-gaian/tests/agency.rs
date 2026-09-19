use gaia_gaian::*;

#[test]
fn ungranted_action_is_denied_and_logged() {
    let mut a = ScopedAgent::default();
    assert_eq!(a.act(Scope::Mail).unwrap_err(), GaianError::NoConsent);
    assert!(a.log().iter().any(|l| l.contains("denied")));
    a.grant(Scope::Mail);
    a.act(Scope::Mail).unwrap();
    a.pause();
    assert_eq!(a.act(Scope::Mail).unwrap_err(), GaianError::NoConsent);
    assert!(!ScopedAgent::paid_amplification());
}

#[test]
fn earth_twin_payload_refuses_raw_biometrics() {
    let a = ScopedAgent::default();
    assert_eq!(
        a.earth_twin_payload(true, false, false).unwrap_err(),
        GaianError::ThirdPartyLikeness
    );
    assert_eq!(a.earth_twin_payload(false, false, false).unwrap(), "aggregate-only fixture");
}

#[test]
fn revoke_stops_email_pay_likeness() {
    let mut agent = Agent::new();
    assert_eq!(agent.act(AgentAct::Email).unwrap_err(), GaianError::GrantRequired);
    agent.revoke();
    assert_eq!(agent.act(AgentAct::Ask).unwrap_err(), GaianError::Revoked);
    assert!(gaian_release_checklist().contains(&"no GAIAN v1.0 tag"));
}

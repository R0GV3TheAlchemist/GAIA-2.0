use gaia_gaian::{GaianError, Scope, ScopedAgent};

#[test]
fn ungranted_is_denied_and_earth_twin_has_no_biometrics() {
    let mut agent = ScopedAgent::default();
    assert_eq!(agent.act(Scope::Mail).unwrap_err(), GaianError::NoConsent);
    agent.grant(Scope::CitizenScience);
    agent.act(Scope::CitizenScience).unwrap();
    assert_eq!(agent.earth_twin_payload(true, false, false).unwrap_err(), GaianError::ThirdPartyLikeness);
    agent.pause();
    assert!(agent.act(Scope::CitizenScience).is_err());
}

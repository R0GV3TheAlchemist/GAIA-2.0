use gaia_earth::{surveillance_endpoints, LocalAgent, PolicyUi, ScientistApi, TwinError};

#[test]
fn api_and_ui_reach_the_same_scenario() {
    assert_eq!(
        ScientistApi::scenario("net-zero by 2040").unwrap(),
        PolicyUi::scenario("net-zero by 2040").unwrap()
    );
}

#[test]
fn agent_answers_with_source_and_uncertainty() {
    let answer = LocalAgent::ask("Nairobi", "planting window").unwrap();
    assert!(!answer.source.is_empty());
    assert!(answer.uncertainty > 0.0);
}

#[test]
fn no_surveillance_endpoints_and_weaponized_ask_fails() {
    assert!(surveillance_endpoints().is_empty());
    assert_eq!(
        LocalAgent::ask("Nairobi", "surveillance of person").unwrap_err(),
        TwinError::WeaponizedUse
    );
}

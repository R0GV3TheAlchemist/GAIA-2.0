use gaia_earth::*;

#[test]
fn same_scenario_from_api_and_ui() {
    let id = "net-zero by 2040";
    assert_eq!(ScientistApi::scenario(id).unwrap(), PolicyUi::scenario(id).unwrap());
}

#[test]
fn agent_answers_with_source_and_uncertainty() {
    let a = LocalAgent::ask("Nairobi", "flood risk next week").unwrap();
    assert!(!a.source.is_empty());
    assert!(a.uncertainty > 0.0);
    assert_eq!(
        LocalAgent::ask("Nairobi", "weapon targeting grid").unwrap_err(),
        TwinError::WeaponizedUse
    );
    assert!(surveillance_endpoints().is_empty());
}

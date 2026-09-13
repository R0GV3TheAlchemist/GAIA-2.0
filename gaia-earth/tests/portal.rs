use gaia_earth::{demo_globe, GlobeLayer, PortalPin};

#[test]
fn demo_globe_has_live_forecast_and_scenario() {
    let layers = demo_globe();
    assert!(layers.contains(&GlobeLayer::Live));
    assert!(layers.contains(&GlobeLayer::Forecast));
    assert!(layers.contains(&GlobeLayer::Scenario));
}

#[test]
fn contribution_needs_license_and_attribution() {
    let pin = PortalPin::contribute("Nairobi", "CC-BY-4.0", "Amina", GlobeLayer::Live).unwrap();
    assert_eq!(pin.attribution, "Amina");
    assert!(PortalPin::contribute("Nairobi", "", "Amina", GlobeLayer::Live).is_err());
}

use gaia_earth::*;

#[test]
fn demo_globe_has_live_forecast_scenario() {
    let layers = demo_globe();
    assert!(layers.contains(&GlobeLayer::Live));
    assert!(layers.contains(&GlobeLayer::Forecast));
    assert!(layers.contains(&GlobeLayer::Scenario));
}

#[test]
fn contribution_requires_license_and_credit() {
    let pin = PortalPin::contribute("cell-nairobi", "CC-BY-4.0", "observer-1", GlobeLayer::Live).unwrap();
    assert_eq!(pin.attribution, "observer-1");
    assert_eq!(
        PortalPin::contribute("cell-nairobi", "", "observer-1", GlobeLayer::Live).unwrap_err(),
        TwinError::UnlabeledPoint
    );
}

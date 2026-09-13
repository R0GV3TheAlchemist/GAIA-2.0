use gaia_earth::{AccessTier, EarthInterface, SourceKind, SystemTwin};

#[test]
fn dashboard_api_and_locale_ask_share_state() {
    let mut earth = EarthInterface::new();
    earth
        .publish("Nairobi", SystemTwin::Atmosphere, SourceKind::Measured, 24.0, 0.6, "degC")
        .unwrap();
    assert_eq!(
        earth.dashboard("Nairobi").unwrap().observation.value,
        earth.scientist_api("Nairobi").unwrap().observation.value
    );
    assert!(earth.ask("Nairobi", "sw").unwrap().contains("locale=sw"));
    assert!(earth.basic_access_is_free("Nairobi").unwrap());
    assert!(EarthInterface::access_tiers().contains(&AccessTier::BasicFree));
}

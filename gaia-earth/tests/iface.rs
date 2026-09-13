//! #38 same state from dashboard, API, and locale ask. Basic access is free.

use gaia_earth::{AccessTier, EarthInterface, SourceKind, SystemTwin};

fn published() -> EarthInterface {
    let mut earth = EarthInterface::new();
    earth
        .publish(
            "Nairobi",
            SystemTwin::Atmosphere,
            SourceKind::Measured,
            24.0,
            0.6,
            "degC",
        )
        .unwrap();
    earth
}

#[test]
fn dashboard_api_and_locale_ask_share_state() {
    let earth = published();
    let dash = earth.dashboard("Nairobi").unwrap();
    let api = earth.scientist_api("Nairobi").unwrap();
    assert_eq!(dash.observation.value, api.observation.value);
    let asked = earth.ask("Nairobi", "sw").unwrap();
    assert!(asked.contains("locale=sw"));
    assert!(asked.contains("place=Nairobi"));
    assert!(asked.contains("24"));
}

#[test]
fn basic_access_has_no_paywall() {
    let earth = published();
    assert!(earth.basic_access_is_free("Nairobi").unwrap());
    assert!(EarthInterface::access_tiers().contains(&AccessTier::BasicFree));
}

#[test]
fn citizen_contribution_needs_license_and_attribution() {
    let mut earth = published();
    let credit = earth
        .contribute("Nairobi", "CC-BY-4.0", "Amina")
        .unwrap();
    assert_eq!(credit.license, "CC-BY-4.0");
    assert!(earth.contribute("Nairobi", "", "Amina").is_err());
}

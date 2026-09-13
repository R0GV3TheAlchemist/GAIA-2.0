use gaia_aispd::{active_weather, start_rsi, wet_lab, Swarm, Watch};

#[test]
fn weather_ok_wet_lab_and_rsi_denied_swarm_small() {
    assert_eq!(active_weather(), Watch::Active);
    assert_eq!(wet_lab(), Watch::Denied);
    assert!(start_rsi().is_err());
    assert!(Swarm::default_bound().size <= 3);
    assert!(Swarm::default_bound().logged);
}

//! #451 / #453 scale-layer bind. Calls APIs already on main.
use gaia_earth::honesty::{
    live_cesium, live_ews_network, live_flink, live_graphcast, live_iceberg, live_kafka,
    live_sentinel_harvest, twin_v1_tagged,
};
use gaia_earth::{allow_purpose, Guardian, Observation, SourceKind, SystemTwin};

#[test]
fn scale_earth_is_record_not_mind() {
    assert!(!twin_v1_tagged());
    assert!(!live_ews_network());
    assert!(!live_graphcast());
    assert!(!live_sentinel_harvest());
    assert!(!live_iceberg());
    assert!(!live_kafka());
    assert!(!live_flink());
    assert!(!live_cesium());
    assert_eq!(SystemTwin::all().len(), 9);
    assert!(Observation::admit(SystemTwin::Ocean, SourceKind::Measured, 1.0, None, "C").is_err());
    assert!(allow_purpose("weapon").is_err());
    let g = Guardian::seed();
    assert_eq!(g.states().len(), 9);
    assert!(g
        .correct(gaia_earth::Boundary::ClimateChange, "recommend")
        .is_err());
}

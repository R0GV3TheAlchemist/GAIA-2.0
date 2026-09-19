use gaia_earth::*;

#[test]
fn nine_boundaries_have_source_timestamp_uncertainty() {
    let mon = BoundaryMonitor::seed(1_700_000_000);
    assert_eq!(mon.states().len(), 9);
    assert!(mon
        .states()
        .iter()
        .all(|s| !s.source.is_empty() && s.timestamp_unix > 0 && s.uncertainty > 0.0));
}

#[test]
fn approach_to_threshold_fires_warning() {
    let mut mon = BoundaryMonitor::seed(1);
    assert!(mon.approach(WatchItem::ClimateChange, 0.85).unwrap());
    assert!(!mon.approach(WatchItem::ClimateChange, 0.2).unwrap());
}

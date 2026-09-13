use gaia_earth::{BoundaryMonitor, WatchItem};

#[test]
fn nine_boundaries_have_source_and_timestamp() {
    let monitor = BoundaryMonitor::seed(1_725_000_000);
    assert_eq!(monitor.states().len(), 9);
    assert!(monitor.states().iter().all(|s| !s.source.is_empty()));
    assert!(monitor.states().iter().all(|s| s.timestamp_unix > 0));
}

#[test]
fn approaching_threshold_fires_warning() {
    let mut monitor = BoundaryMonitor::seed(1);
    assert!(monitor.approach(WatchItem::ClimateChange, 0.85).unwrap());
}

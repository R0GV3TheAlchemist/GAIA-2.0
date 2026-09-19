use gaia_earth::*;

#[test]
fn five_connectors_ingest_bounded_samples() {
    let bus = FeedBus::fixtures();
    let mut lake = Lake::new();
    for c in Connector::all() {
        bus.ingest_sample(&mut lake, c).unwrap();
        assert_eq!(bus.pull_live(c).unwrap_err(), TwinError::FeedUnavailable);
    }
    assert_eq!(lake.curated().len(), 5);
    assert!(lake.curated().iter().all(|r| r.observation.source == SourceKind::Synthetic));
    assert!(lake.curated().iter().all(|r| r.observation.uncertainty > 0.0));
}

#[test]
fn rate_limit_is_visible_not_invented() {
    let mut lim = RateLimit::new(1);
    lim.take().unwrap();
    assert_eq!(lim.take().unwrap_err(), TwinError::FeedUnavailable);
}

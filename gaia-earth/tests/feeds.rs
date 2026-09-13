//! #42 five connectors emit labeled samples. Live pulls do not invent values.

use gaia_earth::{Connector, FeedBus, RateLimit, SourceKind, TwinError};

#[test]
fn each_connector_emits_a_bounded_sample() {
    let bus = FeedBus::fixtures();
    for connector in Connector::all() {
        let record = bus.pull_sample(connector).unwrap();
        assert_eq!(record.connector, connector);
        assert!(!record.license.is_empty());
        assert!(record.provenance.contains("fixture"));
        assert_eq!(record.observation.source, SourceKind::Synthetic);
        assert!(record.observation.uncertainty > 0.0);
        assert_eq!(record.topic, connector.topic());
    }
}

#[test]
fn live_pull_fails_visibly() {
    let err = FeedBus::fixtures()
        .pull_live(Connector::Copernicus)
        .unwrap_err();
    assert_eq!(err, TwinError::FeedUnavailable);
    assert!(format!("{err}").contains("not invented"));
}

#[test]
fn rate_limit_blocks_a_second_pull() {
    let mut limit = RateLimit::new(1);
    limit.take().unwrap();
    assert_eq!(limit.take().unwrap_err(), TwinError::FeedUnavailable);
}

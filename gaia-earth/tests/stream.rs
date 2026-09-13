//! #43 synthetic station feed survives a checkpoint. Lag is visible.

use gaia_earth::{SourceKind, StreamBus, SystemTwin, TwinError};

#[test]
fn valid_event_is_consumed_and_lag_drops() {
    let mut bus = StreamBus::new();
    bus.publish(
        SystemTwin::Atmosphere,
        SourceKind::Measured,
        22.0,
        Some(0.3),
        "degC",
    )
    .unwrap();
    assert_eq!(bus.metrics().lag(), 1);
    let event = bus.consume().unwrap();
    assert_eq!(event.topic, "stream.atmosphere");
    assert_eq!(event.quality, "L1Screened");
    assert_eq!(bus.metrics().lag(), 0);
}

#[test]
fn poison_message_lands_on_the_dead_letter_queue() {
    let mut bus = StreamBus::new();
    let err = bus
        .publish(
            SystemTwin::Ocean,
            SourceKind::Measured,
            1.0,
            None,
            "degC",
        )
        .unwrap_err();
    assert_eq!(err, TwinError::MissingUncertainty);
    assert_eq!(bus.dead_letters().len(), 1);
    assert_eq!(bus.metrics().dead_letters, 1);
    assert!(bus.consume().is_none());
}

#[test]
fn checkpoint_survives_restart_without_silent_loss() {
    let mut bus = StreamBus::new();
    bus.publish(
        SystemTwin::Land,
        SourceKind::Synthetic,
        0.4,
        Some(0.1),
        "ndvi",
    )
    .unwrap();
    let saved = bus.checkpoint();
    let mut restored = StreamBus::restore(saved);
    assert_eq!(restored.metrics().published, 1);
    assert!(restored.consume().is_some());
}

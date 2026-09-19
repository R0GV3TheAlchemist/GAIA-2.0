use gaia_earth::*;

#[test]
fn poison_goes_to_dead_letter_not_invented() {
    let mut bus = StreamBus::new();
    assert_eq!(
        bus.publish(
            SystemTwin::Atmosphere,
            SourceKind::Measured,
            12.0,
            None,
            "degC"
        )
        .unwrap_err(),
        TwinError::MissingUncertainty
    );
    assert_eq!(bus.dead_letters().len(), 1);
    assert_eq!(bus.metrics().dead_letters, 1);
}

#[test]
fn checkpoint_survives_restart_without_silent_loss() {
    let mut bus = StreamBus::new();
    bus.publish(
        SystemTwin::Atmosphere,
        SourceKind::Synthetic,
        21.0,
        Some(0.3),
        "degC",
    )
    .unwrap();
    bus.publish(
        SystemTwin::Ocean,
        SourceKind::Synthetic,
        18.0,
        Some(0.2),
        "degC",
    )
    .unwrap();
    assert_eq!(bus.metrics().lag(), 2);
    let saved = bus.checkpoint();
    let mut restored = StreamBus::restore(saved);
    assert_eq!(restored.consume().unwrap().topic, "stream.atmosphere");
    assert_eq!(restored.consume().unwrap().topic, "stream.ocean");
    assert!(restored.consume().is_none());
    assert_eq!(restored.metrics().lag(), 0);
    assert_eq!(restored.metrics().published, 2);
    assert_eq!(restored.metrics().consumed, 2);
}

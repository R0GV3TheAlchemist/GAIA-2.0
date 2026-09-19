use gaia_earth::*;

#[test]
fn citizen_pin_has_license_and_credit() {
    let rec = LivingRecord::contribute(
        LivingKind::Citizen,
        "cell-austin",
        "Quercus",
        "CC-BY-4.0",
        "observer-1",
        None,
        None,
    )
    .unwrap();
    let pin = rec.map_pin();
    assert!(pin.contains("license=CC-BY-4.0"));
    assert!(pin.contains("credit=observer-1"));
}

#[test]
fn acoustic_requires_model_and_confidence() {
    assert_eq!(
        LivingRecord::contribute(
            LivingKind::Acoustic,
            "cell-austin",
            "Turdus",
            "CC-BY-4.0",
            "observer-1",
            None,
            None,
        )
        .unwrap_err(),
        TwinError::UnlabeledPoint
    );
    let rec = LivingRecord::contribute(
        LivingKind::Acoustic,
        "cell-austin",
        "Turdus",
        "CC-BY-4.0",
        "observer-1",
        Some("birdnet-fixture-0"),
        Some(0.81),
    )
    .unwrap();
    assert_eq!(rec.model_version.as_deref(), Some("birdnet-fixture-0"));
    assert_eq!(rec.confidence, Some(0.81));
}

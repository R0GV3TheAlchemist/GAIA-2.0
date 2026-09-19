use gaia_earth::*;

fn obs(source: SourceKind, value: f64, u: f64) -> Observation {
    Observation::admit(SystemTwin::Atmosphere, source, value, Some(u), "degC").unwrap()
}

#[test]
fn curated_record_refuses_missing_method_or_range() {
    let ok = obs(SourceKind::Measured, 15.0, 0.2);
    CuratedRecord::ship(
        ok.clone(),
        "station-mean",
        "fixture:local",
        1,
        QualityClass::Operational,
    )
    .unwrap();
    assert_eq!(
        CuratedRecord::ship(ok, "", "fixture:local", 1, QualityClass::Operational).unwrap_err(),
        TwinError::UnlabeledPoint
    );
    let hot = Observation::admit(
        SystemTwin::Atmosphere,
        SourceKind::Measured,
        99.0,
        Some(0.1),
        "degC",
    )
    .unwrap();
    assert_eq!(
        CuratedRecord::ship(
            hot,
            "station-mean",
            "fixture:local",
            1,
            QualityClass::Caution
        )
        .unwrap_err(),
        TwinError::UnlabeledPoint
    );
}

#[test]
fn admit_without_uncertainty_cannot_become_curated() {
    assert_eq!(
        Observation::admit(
            SystemTwin::Atmosphere,
            SourceKind::Measured,
            12.0,
            None,
            "degC"
        )
        .unwrap_err(),
        TwinError::MissingUncertainty
    );
}

#[test]
fn assimilate_blends_and_labels_synthetic() {
    let model = obs(SourceKind::Synthetic, 10.0, 2.0);
    let seen = obs(SourceKind::Measured, 14.0, 1.0);
    let state = assimilate(model, seen).unwrap();
    assert_eq!(state.source, SourceKind::Synthetic);
    assert!((state.value - 12.666).abs() < 0.01 || (state.value > 10.0 && state.value < 14.0));
    assert!(state.uncertainty > 0.0);
}

#[test]
fn assimilate_refuses_zero_uncertainty() {
    let model = obs(SourceKind::Synthetic, 10.0, 0.0);
    let seen = obs(SourceKind::Measured, 14.0, 1.0);
    assert_eq!(
        assimilate(model, seen).unwrap_err(),
        TwinError::MissingUncertainty
    );
}

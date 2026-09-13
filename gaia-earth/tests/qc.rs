use gaia_earth::{assimilate, CuratedRecord, Observation, QualityClass, SourceKind, SystemTwin};

fn obs(source: SourceKind, value: f64, u: f64) -> Observation {
    Observation::admit(SystemTwin::Atmosphere, source, value, Some(u), "degC").unwrap()
}

#[test]
fn curated_record_requires_uncertainty_and_quality() {
    let record = CuratedRecord::ship(
        obs(SourceKind::Measured, 20.0, 0.3),
        "range-check",
        "fixture station",
        1,
        QualityClass::Operational,
    )
    .unwrap();
    assert_eq!(record.quality, QualityClass::Operational);
    assert!(record.observation.uncertainty > 0.0);
}

#[test]
fn assimilation_blends_model_and_observation() {
    let blended = assimilate(
        obs(SourceKind::Synthetic, 10.0, 2.0),
        obs(SourceKind::Measured, 20.0, 1.0),
    )
    .unwrap();
    assert_eq!(blended.source, SourceKind::Synthetic);
    assert!(blended.value > 10.0 && blended.value < 20.0);
}

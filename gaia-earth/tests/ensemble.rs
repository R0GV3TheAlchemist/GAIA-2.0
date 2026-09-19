use gaia_earth::*;

#[test]
fn two_models_write_synthetic_dated_products() {
    let mut lake = Lake::new();
    let a = ModelProduct::infer(RegisteredModel::GraphCast, SystemTwin::Atmosphere).unwrap();
    let b = ModelProduct::infer(RegisteredModel::TerraMind, SystemTwin::Land).unwrap();
    assert!(!a.is_observation());
    assert!(!b.is_observation());
    assert_eq!(a.dated, "2026-09-19");
    a.into_lake(&mut lake).unwrap();
    b.into_lake(&mut lake).unwrap();
    assert_eq!(lake.curated().len(), 2);
    assert!(lake.curated().iter().all(|r| r.observation.source == SourceKind::Synthetic));
}

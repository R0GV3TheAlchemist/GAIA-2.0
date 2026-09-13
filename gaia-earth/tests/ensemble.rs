use gaia_earth::{ModelProduct, RegisteredModel, SystemTwin};

#[test]
fn two_models_emit_synthetic_dated_products() {
    let a = ModelProduct::infer(RegisteredModel::GraphCast, SystemTwin::Atmosphere).unwrap();
    let b = ModelProduct::infer(RegisteredModel::TerraMind, SystemTwin::Land).unwrap();
    assert!(!a.is_observation());
    assert!(!b.is_observation());
    assert_eq!(a.version, "fixture-0");
}

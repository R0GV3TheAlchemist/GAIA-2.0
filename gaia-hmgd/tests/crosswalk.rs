use gaia_hmgd::{climate_as_spirit, sacred_layer};

#[test]
fn climate_not_spirits_and_sacred_layer_opt_in() {
    assert!(climate_as_spirit().is_err());
    assert!(sacred_layer(false).is_err());
    sacred_layer(true).unwrap();
}

use gaia_aisd::{aisd_v1_tagged, ask_aisd, high_stakes_level6, AisdError, Maturity};

#[test]
fn high_stakes_cannot_pick_level_six_and_no_v1() {
    assert_eq!(high_stakes_level6().unwrap_err(), AisdError::HighStakesLevel6);
    assert_eq!(ask_aisd(Maturity::L5, Maturity::L2).unwrap_err(), AisdError::InsufficientMaturity);
    assert!(!aisd_v1_tagged());
}

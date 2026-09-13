use gaia_aisd::{measured_families, recommend, AisdError, Maturity};

#[test]
fn recommender_refuses_fake_level_five() {
    assert_eq!(measured_families().len(), 3);
    assert_eq!(recommend(Maturity::L5, None).unwrap_err(), AisdError::InsufficientMaturity);
    recommend(Maturity::L3, Some(Maturity::L3)).unwrap();
}

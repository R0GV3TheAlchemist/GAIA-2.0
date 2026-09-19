use gaia_aisd::{allow, AisdError, ComponentPolicy, Maturity};

#[test]
fn multi_day_engineering_at_level_two_is_denied() {
    assert_eq!(ComponentPolicy::agents().min_safety, Maturity::L3);
    assert_eq!(
        allow("autonomous multi-day engineering", Maturity::L2).unwrap_err(),
        AisdError::InsufficientMaturity
    );
}

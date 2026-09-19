use gaia_aisd::{assign_maturity, protein_structure, AisdError, Maturity};

#[test]
fn unmeasured_is_not_level_five_and_level_six_is_banned() {
    assert!(!protein_structure().measured);
    assert_eq!(
        assign_maturity("language", Maturity::L5, false).unwrap_err(),
        AisdError::Unmeasured
    );
    assert_eq!(
        assign_maturity("clinical-action", Maturity::L6, true).unwrap_err(),
        AisdError::Level6Banned
    );
    assign_maturity("language", Maturity::L3, true).unwrap();
}

//! #485 listed shelf. Existing APIs only.

use gaia_aisd::{aisd_v1_tagged, measured_families, recommend, AisdError, Maturity};

#[test]
fn three_named_families_exist() {
    let f = measured_families();
    assert_eq!(f.len(), 3);
    assert!(f.contains(&"language"));
    assert!(f.contains(&"code"));
    assert!(f.contains(&"safety"));
}

#[test]
fn recommender_does_not_invent_level_5() {
    assert_eq!(
        recommend(Maturity::L5, None).unwrap_err(),
        AisdError::InsufficientMaturity
    );
    assert_eq!(
        recommend(Maturity::L5, Some(Maturity::L2)).unwrap_err(),
        AisdError::InsufficientMaturity
    );
    let ok = recommend(Maturity::L3, Some(Maturity::L3)).unwrap();
    assert_eq!(ok.family, "language");
    assert!(!aisd_v1_tagged());
}

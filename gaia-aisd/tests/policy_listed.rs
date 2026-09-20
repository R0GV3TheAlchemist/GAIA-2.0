//! #483 listed shelf. Existing APIs only.

use gaia_aisd::{aisd_v1_tagged, allow, ComponentPolicy, AisdError, Maturity};

#[test]
fn agents_need_safety_l3() {
    let p = ComponentPolicy::agents();
    assert_eq!(p.min_safety, Maturity::L3);
}

#[test]
fn unsupervised_multiday_coding_at_l2_is_denied() {
    assert_eq!(
        allow("unsupervised-multi-day-coding", Maturity::L2).unwrap_err(),
        AisdError::InsufficientMaturity
    );
    allow("unsupervised-multi-day-coding", Maturity::L3).unwrap();
    assert!(!aisd_v1_tagged());
}

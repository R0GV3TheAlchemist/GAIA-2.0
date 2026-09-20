//! #571 listed shelf. Existing APIs only.

use gaia_aikd::{aikd_v1_tagged, Adapter, Executed, Tier};

#[test]
fn passing_tests_are_t1_failures_are_t4() {
    let ok = Executed::code(true, None);
    assert_eq!(ok.tier, Tier::T1);
    assert!(ok.error.is_none());
    let bad = Executed::code(false, Some("boom"));
    assert_eq!(bad.tier, Tier::T4);
    assert_eq!(bad.error.as_deref(), Some("boom"));
}

#[test]
fn unverified_proof_is_not_t1() {
    let p = Executed::proof(false);
    assert_eq!(p.tier, Tier::T4);
    assert_eq!(p.error.as_deref(), Some("unverified proof"));
    assert_eq!(Adapter::Code.eval_slice(), "code-eval-slice");
    assert!(!aikd_v1_tagged());
}

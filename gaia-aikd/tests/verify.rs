use gaia_aikd::{Executed, Tier};

#[test]
fn passing_tests_are_tier_one_failures_stay_tier_four() {
    assert_eq!(Executed::code(true, None).tier, Tier::T1);
    let fail = Executed::code(false, Some("boom"));
    assert_eq!(fail.tier, Tier::T4);
    assert_eq!(fail.error.as_deref(), Some("boom"));
    assert_eq!(Executed::proof(false).tier, Tier::T4);
}

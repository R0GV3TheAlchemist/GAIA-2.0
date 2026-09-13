use gaia_aikd::{AikdError, Generation, HallucinationKind, Tier};

#[test]
fn fixtures_are_flagged_and_tier_five_is_not_a_bare_fact() {
    let gen = Generation::from_fixture("capital of france is berlin", Tier::T3).unwrap();
    assert!(gen.flags.contains(&HallucinationKind::Factual));
    assert_eq!(
        Generation::from_fixture("the sky is blue", Tier::T5).unwrap_err(),
        AikdError::NeedVerify
    );
    Generation::from_fixture("verify: uncertain", Tier::T5).unwrap();
}

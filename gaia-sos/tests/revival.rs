use gaia_sos::{adopted_module_count, live_mcp, live_owm, map_concept, maturity, mcre_suite_restored, refuse_archive_exec, refuse_consciousness_runtime, second_kernel, sentience, virelai_736_claimed, Grade};

#[test]
fn transfer_is_evidence_not_a_kernel() {
    assert!(!second_kernel());
    assert!(!sentience());
    assert!(!virelai_736_claimed());
    assert!(adopted_module_count() < 736);
    assert!(!mcre_suite_restored());
    assert!(!live_owm());
    assert!(!live_mcp());
}

#[test]
fn reusable_patterns_map_to_existing_crates() {
    let hal = map_concept("HAL").unwrap();
    assert_eq!(hal.gaia, "gaia-sos HalTier T0-T4");
    assert_eq!(hal.grade, Grade::Specified);
    assert_eq!(map_concept("orchestrator").unwrap().grade, Grade::Tested);
    assert_eq!(maturity("gaia-orchestrator"), Grade::Tested);
    assert_eq!(maturity("gaia-kernel"), Grade::Tested);
}

#[test]
fn missing_assets_and_runtime_claims_are_refused() {
    assert!(refuse_consciousness_runtime().is_err());
    assert!(refuse_archive_exec().is_err());
    assert_eq!(map_concept("MCRE-2000").unwrap().grade, Grade::Conceptual);
    assert_eq!(map_concept("weather-OWM").unwrap().grade, Grade::Conceptual);
}

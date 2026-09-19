//! #477 listed shelf. Existing APIs only.

use gaia_aisd::{
    aisd_v1_tagged, graphcast, high_stakes_level6, reference_published, AisdError,
};

#[test]
fn graphcast_is_a_tool_not_an_llm_card() {
    let g = graphcast();
    assert_eq!(g.name, "GraphCast");
    assert_eq!(g.kind, "weather-forecast-tool");
    assert!(!g.is_llm_card);
}

#[test]
fn published_scores_are_reference_and_l6_high_stakes_banned() {
    let row = reference_published("MMLU", "0.00");
    assert!(row.contains("reference_published"));
    assert!(row.contains("gaia_measured="));
    assert!(!row.contains("gaia_measured=1"));
    assert_eq!(high_stakes_level6().unwrap_err(), AisdError::HighStakesLevel6);
    assert!(!aisd_v1_tagged());
}

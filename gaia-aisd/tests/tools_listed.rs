//! #479 listed shelf. Existing APIs only.

use gaia_aisd::{graphcast, reference_published};

#[test]
fn graphcast_links_weather_skill_and_is_not_an_llm() {
    let g = graphcast();
    assert_eq!(g.skill, "aisd:science:weather-forecast");
    assert_eq!(g.kind, "weather-forecast-tool");
    assert!(!g.is_llm_card);
}

#[test]
fn published_bench_is_not_gaia_measured() {
    let elo = reference_published("Elo", "1200");
    let mmlu = reference_published("MMLU", "0.00");
    assert!(elo.starts_with("Elo=reference_published:"));
    assert!(mmlu.contains("gaia_measured="));
    assert!(!mmlu.contains("gaia_measured=0.00"));
}

use gaia_earth::{
    OutcomeKind, ScenarioEngine, ScenarioLibrary, SimError, SimMode, SourceKind,
};

#[test]
fn what_if_net_zero_returns_ensemble_with_uncertainty() {
    let run = ScenarioEngine::new().run("net-zero by 2040").unwrap();
    assert_eq!(run.mode, SimMode::WhatIf);
    assert_eq!(run.outcomes.len(), 5);
    assert!(run.outcomes.iter().any(|d| d.kind == OutcomeKind::TippingRisk));
    assert!(run.outcomes.iter().all(|d| d.uncertainty > 0.0));
    assert!(run.outcomes.iter().all(|d| d.source == SourceKind::Synthetic));
}

#[test]
fn unknown_scenario_is_refused() {
    assert_eq!(
        ScenarioEngine::new().run("invented world").unwrap_err(),
        SimError::UnknownScenario
    );
}

#[test]
fn canned_scenarios_return_synthetic_distributions_with_attribution() {
    for scenario in ScenarioLibrary::canned() {
        let rows = ScenarioLibrary::run(scenario).unwrap();
        assert_eq!(rows.len(), 5);
        assert!(rows.iter().all(|row| row.source == SourceKind::Synthetic));
        assert!(rows.iter().all(|row| row.uncertainty > 0.0));
        assert!(rows.iter().all(|row| row.model == "fixture-ensemble-0"));
        assert!(rows
            .iter()
            .all(|row| row.uncertainty_method == "fixture-distribution"));
    }
}

#[test]
fn unknown_library_scenario_is_rejected() {
    assert!(ScenarioLibrary::run("invented world").is_err());
}

#[test]
fn fixture_cascade_is_explicit() {
    let edges = ScenarioLibrary::cascade();
    assert_eq!(edges[0].from, "AMOC");
    assert_eq!(edges[0].to, "Amazon");
    assert_eq!(edges[1].from, "Amazon");
    assert_eq!(edges[1].to, "rainfall");
}

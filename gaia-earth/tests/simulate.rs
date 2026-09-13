use gaia_earth::{OutcomeKind, ScenarioEngine, SimError, SimMode, SourceKind};

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

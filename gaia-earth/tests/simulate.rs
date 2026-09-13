//! #37 What-If returns five labeled distributions with uncertainty.

use gaia_earth::{OutcomeKind, ScenarioEngine, SimError, SimMode, SourceKind};

#[test]
fn what_if_net_zero_returns_ensemble_with_uncertainty() {
    let engine = ScenarioEngine::new();
    let run = engine.run("net-zero by 2040").unwrap();
    assert_eq!(run.mode, SimMode::WhatIf);
    assert_eq!(run.outcomes.len(), 5);
    assert!(run.outcomes.iter().any(|d| d.kind == OutcomeKind::Climate));
    assert!(run.outcomes.iter().any(|d| d.kind == OutcomeKind::Biodiversity));
    assert!(run.outcomes.iter().any(|d| d.kind == OutcomeKind::Economy));
    assert!(run.outcomes.iter().any(|d| d.kind == OutcomeKind::Welfare));
    assert!(run.outcomes.iter().any(|d| d.kind == OutcomeKind::TippingRisk));
    assert!(run.outcomes.iter().all(|d| d.uncertainty > 0.0));
    assert!(run.outcomes.iter().all(|d| d.source == SourceKind::Synthetic));
}

#[test]
fn amoc_scenario_is_in_the_library() {
    let engine = ScenarioEngine::new();
    assert!(engine.library().iter().any(|(id, _)| *id == "AMOC -30%"));
    let run = engine.run("AMOC -30%").unwrap();
    assert_eq!(run.mode, SimMode::WhatIf);
}

#[test]
fn unknown_scenario_is_refused() {
    let err = ScenarioEngine::new().run("invented world").unwrap_err();
    assert_eq!(err, SimError::UnknownScenario);
}

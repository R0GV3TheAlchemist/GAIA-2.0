//! #21 depth: 3+ node DAG, inspect-before-run, topo, cycle refuse, fallback.

use gaia_memos::MemOs;
use gaia_orchestrator::{Executor, IntentEngine, TaskPlanner};

fn research_plan() -> (gaia_orchestrator::Plan, MemOs) {
    let mut mem = MemOs::new();
    let engine = IntentEngine::local_stub();
    let g = engine.parse("research and summarize", &mut mem).unwrap();
    (TaskPlanner::from_intent(&g), mem)
}

#[test]
fn research_and_summarize_is_three_plus_nodes() {
    let (plan, _) = research_plan();
    assert!(plan.nodes.len() >= 3);
    assert!(!plan.accepted);
    let view = plan.inspect();
    assert!(view.contains("inspect-before-run"));
    assert!(view.contains("retriever") || view.contains("researcher"));
    assert!(view.contains("fallback="));
    assert!(view.contains("retries="));
    plan.topo_order().unwrap();
}

#[test]
fn execute_refused_until_accepted() {
    let (plan, mut mem) = research_plan();
    let ex = Executor::default();
    assert!(ex.execute(&plan, &mut mem).is_err());
}

#[test]
fn cycle_is_refused() {
    let (mut plan, mut mem) = research_plan();
    plan.accept();
    let a = plan.nodes[0].id;
    let b = plan.nodes[1].id;
    plan.nodes[0].depends_on = vec![b];
    plan.nodes[1].depends_on = vec![a];
    let ex = Executor::default();
    let err = ex.execute(&plan, &mut mem).unwrap_err();
    assert!(err.contains("cycle"));
}

#[test]
fn failed_node_retries_then_fallback() {
    let (mut plan, mut mem) = research_plan();
    plan.accept();
    let research = plan
        .nodes
        .iter()
        .find(|n| n.goal.contains("research:"))
        .unwrap();
    let budget = research.max_retries + 1;
    let research_id = research.id;

    let ex = Executor {
        fail_goals: vec!["research:".into()],
        ..Executor::default()
    };
    let report = ex.execute(&plan, &mut mem).unwrap();
    assert!(report.used_fallback.contains(&research_id));
    let attempt = report
        .attempts
        .iter()
        .find(|a| a.node_id == research_id)
        .unwrap();
    assert_eq!(attempt.primary_attempts, budget);
    assert!(attempt.used_fallback);
    assert!(report.outputs.iter().any(|o| o.starts_with("retry:")));
    assert!(report.outputs.iter().any(|o| o.starts_with("fallback:")));
    assert!(report.plan_cube_id.is_some());
    assert!(report.cube_id.is_some());
}

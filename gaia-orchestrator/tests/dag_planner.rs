//! #21 Task Planner: 3+ node DAG, inspect-before-run, fallback, MemOS persist.

use gaia_memos::MemOs;
use gaia_orchestrator::{Executor, IntentEngine, TaskPlanner};

fn research_plan() -> (gaia_orchestrator::Plan, MemOs) {
    let mem = MemOs::new();
    let engine = IntentEngine::local_stub();
    let g = engine.parse("research and summarize", &mem).unwrap();
    (TaskPlanner::from_intent(&g), mem)
}

#[test]
fn research_and_summarize_is_three_plus_nodes() {
    let (plan, _) = research_plan();
    assert!(plan.nodes.len() >= 3);
    assert!(!plan.accepted);
    let view = plan.inspect();
    assert!(view.contains("retriever") || view.contains("researcher"));
    assert!(view.contains("fallback="));
}

#[test]
fn execute_refused_until_accepted() {
    let (plan, mut mem) = research_plan();
    let ex = Executor::default();
    assert!(ex.execute(&plan, &mut mem).is_err());
}

#[test]
fn failed_node_triggers_fallback() {
    let (mut plan, mut mem) = research_plan();
    plan.accept();
    let ex = Executor {
        fail_goals: vec!["research:".into()],
        ..Executor::default()
    };
    let report = ex.execute(&plan, &mut mem).unwrap();
    assert!(!report.used_fallback.is_empty());
    assert!(report.outputs.iter().any(|o| o.starts_with("fallback:")));
    assert!(report.cube_id.is_some());
    let cube = mem.get(report.cube_id.unwrap()).unwrap();
    assert!(cube.content.contains("fallback"));
}

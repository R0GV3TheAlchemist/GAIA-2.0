//! #4 approved local execution: verify before run; death → requeue → audit.

use gaia_memos::MemOs;
use gaia_orchestrator::{Broker, IntentEngine, IntentSigner, LocalRunner, TaskPlanner, TrustAudit};

fn graph_and_plan() -> (gaia_orchestrator::IntentGraph, gaia_orchestrator::Plan) {
    let mut mem = MemOs::new();
    let graph = IntentEngine::local_stub()
        .parse("research and summarize CARE", &mut mem)
        .unwrap();
    let plan = TaskPlanner::from_intent(&graph);
    (graph, plan)
}

#[test]
fn unaccepted_plan_is_refused() {
    let (graph, plan) = graph_and_plan();
    let signed = IntentSigner::generate().sign(&graph);
    let mut broker = Broker::new();
    let mut audit = TrustAudit::default();
    assert!(LocalRunner::run(&plan, &mut broker, &mut audit, &signed, None).is_err());
    assert!(audit.events().is_empty());
}

#[test]
fn tampered_intent_is_refused_before_dispatch() {
    let (graph, mut plan) = graph_and_plan();
    let mut signed = IntentSigner::generate().sign(&graph);
    plan.accept();
    signed.canonical_payload.push_str("tampered");
    let mut broker = Broker::new();
    let mut audit = TrustAudit::default();
    let err = LocalRunner::run(&plan, &mut broker, &mut audit, &signed, None).unwrap_err();
    assert!(err.contains("tampered") || err.contains("rejected"));
    assert!(audit.events().is_empty());
}

#[test]
fn accepted_plan_runs_all_local_nodes_and_audits_lifecycle() {
    let (graph, mut plan) = graph_and_plan();
    let signed = IntentSigner::generate().sign(&graph);
    plan.accept_verified(&signed).unwrap();
    let mut broker = Broker::new();
    let mut audit = TrustAudit::default();
    let run = LocalRunner::run(&plan, &mut broker, &mut audit, &signed, None).unwrap();
    assert_eq!(run.completed_jobs.len(), plan.nodes.len());
    assert!(run.failed_over_jobs.is_empty());
    assert_eq!(audit.events().len(), plan.nodes.len() * 2);
    assert!(audit.chain_ok());
    assert!(audit.events().iter().all(|e| e.intent_id == plan.intent_id));
    assert!(audit.events().iter().all(|e| e.plan_id == Some(plan.id)));
    assert!(audit.events().iter().any(|e| e.event.starts_with("node-started:")));
    assert!(audit.events().iter().any(|e| e.event.starts_with("node-completed:")));
}

#[test]
fn killed_executor_requeues_and_remaining_specialist_completes() {
    let (graph, mut plan) = graph_and_plan();
    let signed = IntentSigner::generate().sign(&graph);
    plan.accept_verified(&signed).unwrap();
    let mut broker = Broker::new();
    let mut audit = TrustAudit::default();
    let run = LocalRunner::run(&plan, &mut broker, &mut audit, &signed, Some("specialist-a")).unwrap();
    assert_eq!(run.completed_jobs.len(), plan.nodes.len());
    assert_eq!(run.failed_over_jobs.len(), 1);
    assert!(!broker.workers.iter().find(|w| w.id == "specialist-a").unwrap().alive);
    assert!(audit.events().iter().any(|e| e.event.starts_with("node-failed-over:")));
    assert!(audit.events().iter().any(|e| {
        e.executor_id.as_deref() == Some("specialist-b") && e.event.starts_with("node-completed:")
    }));
}

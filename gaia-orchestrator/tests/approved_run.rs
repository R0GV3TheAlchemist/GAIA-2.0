//! #4 approved local execution: no approval, no run; death → requeue → audit.

use gaia_memos::MemOs;
use gaia_orchestrator::{Broker, IntentEngine, LocalRunner, TaskPlanner, TrustAudit};

fn plan() -> gaia_orchestrator::Plan {
    let mem = MemOs::new();
    let graph = IntentEngine::local_stub()
        .parse("research and summarize CARE", &mem)
        .unwrap();
    TaskPlanner::from_intent(&graph)
}

#[test]
fn unaccepted_plan_is_refused() {
    let plan = plan();
    let mut broker = Broker::new();
    let mut audit = TrustAudit::default();
    assert!(LocalRunner::run(&plan, &mut broker, &mut audit, None).is_err());
    assert!(audit.events().is_empty());
}

#[test]
fn accepted_plan_runs_all_local_nodes_and_audits_lifecycle() {
    let mut plan = plan();
    plan.accept();
    let mut broker = Broker::new();
    let mut audit = TrustAudit::default();
    let run = LocalRunner::run(&plan, &mut broker, &mut audit, None).unwrap();
    assert_eq!(run.completed_jobs.len(), plan.nodes.len());
    assert!(run.failed_over_jobs.is_empty());
    assert_eq!(audit.events().len(), plan.nodes.len() * 2);
    assert!(audit.events().iter().all(|e| e.intent_id == plan.intent_id));
    assert!(audit.events().iter().all(|e| e.plan_id == Some(plan.id)));
    assert!(audit.events().iter().any(|e| e.event.starts_with("node-started:")));
    assert!(audit.events().iter().any(|e| e.event.starts_with("node-completed:")));
}

#[test]
fn killed_executor_requeues_and_remaining_specialist_completes() {
    let mut plan = plan();
    plan.accept();
    let mut broker = Broker::new();
    let mut audit = TrustAudit::default();
    let run = LocalRunner::run(&plan, &mut broker, &mut audit, Some("specialist-a")).unwrap();
    assert_eq!(run.completed_jobs.len(), plan.nodes.len());
    assert_eq!(run.failed_over_jobs.len(), 1);
    assert!(!broker.workers.iter().find(|w| w.id == "specialist-a").unwrap().alive);
    assert!(audit.events().iter().any(|e| e.event.starts_with("node-failed-over:")));
    assert!(audit.events().iter().any(|e| {
        e.executor_id.as_deref() == Some("specialist-b") && e.event.starts_with("node-completed:")
    }));
}

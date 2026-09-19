//! #4 approved local execution: verify before run; death → requeue → audit.
//! #335: gate is checked before queueing or node dispatch.

use gaia_memos::MemOs;
use gaia_orchestrator::{
    Broker, InMemoryGate, InMemorySink, IntentEngine, IntentSigner, LocalRunner, TaskPlanner,
    TraceEventKind, TrustAudit,
};

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
    assert!(broker.queue.is_empty());
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
    assert!(audit
        .events()
        .iter()
        .any(|e| e.event.starts_with("node-started:")));
    assert!(audit
        .events()
        .iter()
        .any(|e| e.event.starts_with("node-completed:")));
}

#[test]
fn killed_executor_requeues_and_remaining_specialist_completes() {
    let (graph, mut plan) = graph_and_plan();
    let signed = IntentSigner::generate().sign(&graph);
    plan.accept_verified(&signed).unwrap();
    let mut broker = Broker::new();
    let mut audit = TrustAudit::default();
    let run = LocalRunner::run(
        &plan,
        &mut broker,
        &mut audit,
        &signed,
        Some("specialist-a"),
    )
    .unwrap();
    assert_eq!(run.completed_jobs.len(), plan.nodes.len());
    assert_eq!(run.failed_over_jobs.len(), 1);
    assert!(
        !broker
            .workers
            .iter()
            .find(|w| w.id == "specialist-a")
            .unwrap()
            .alive
    );
    assert!(audit
        .events()
        .iter()
        .any(|e| e.event.starts_with("node-failed-over:")));
    assert!(audit.events().iter().any(|e| {
        e.executor_id.as_deref() == Some("specialist-b") && e.event.starts_with("node-completed:")
    }));
}

#[test]
fn active_gap_lock_blocks_before_queue_or_node_dispatch() {
    let (graph, mut plan) = graph_and_plan();
    let signed = IntentSigner::generate().sign(&graph);
    plan.accept_verified(&signed).unwrap();
    let gate = InMemoryGate::locked("gap-42");
    let sink = InMemorySink::new();
    let mut broker = Broker::new();
    let mut audit = TrustAudit::with_sink(Box::new(sink.clone()));

    let err =
        LocalRunner::run_with_gate(&plan, &mut broker, &mut audit, &signed, &gate, false, None)
            .unwrap_err();

    assert!(err.contains("GAIA_GAP_LOCK_ACTIVE"));
    assert!(broker.queue.is_empty());
    assert!(audit.events().is_empty());
    assert_eq!(audit.kernel_len(), 0);
    assert!(audit.chain_ok());
    let events = sink.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].kind, TraceEventKind::ExecutionBlockedByGapLock);
    assert_eq!(events[0].reason_code, "GAIA_GAP_LOCK_ACTIVE");
}

#[test]
fn local_dev_control_plane_unavailability_audits_and_runs() {
    let (graph, mut plan) = graph_and_plan();
    let signed = IntentSigner::generate().sign(&graph);
    plan.accept_verified(&signed).unwrap();
    let gate = InMemoryGate::unavailable();
    let mut broker = Broker::new();
    let mut audit = TrustAudit::default();

    let run =
        LocalRunner::run_with_gate(&plan, &mut broker, &mut audit, &signed, &gate, false, None)
            .unwrap();

    assert_eq!(run.completed_jobs.len(), plan.nodes.len());
    assert!(audit
        .events()
        .iter()
        .any(|e| e.event.starts_with("node-started:")));
    assert_eq!(audit.kernel_len(), plan.nodes.len() * 2);
    assert!(audit.chain_ok());
}

#[test]
fn deployed_control_plane_unavailability_blocks_before_dispatch() {
    let (graph, mut plan) = graph_and_plan();
    let signed = IntentSigner::generate().sign(&graph);
    plan.accept_verified(&signed).unwrap();
    let gate = InMemoryGate::unavailable();
    let sink = InMemorySink::new();
    let mut broker = Broker::new();
    let mut audit = TrustAudit::with_sink(Box::new(sink.clone()));

    let err =
        LocalRunner::run_with_gate(&plan, &mut broker, &mut audit, &signed, &gate, true, None)
            .unwrap_err();

    assert!(err.contains("GAIA_CONTROL_PLANE_UNAVAILABLE"));
    assert!(broker.queue.is_empty());
    assert!(audit.events().is_empty());
    assert_eq!(audit.kernel_len(), 0);
    assert!(audit.chain_ok());
    let events = sink.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].kind, TraceEventKind::ControlPlaneUnavailable);
    assert_eq!(events[0].reason_code, "GAIA_CONTROL_PLANE_UNAVAILABLE");
}

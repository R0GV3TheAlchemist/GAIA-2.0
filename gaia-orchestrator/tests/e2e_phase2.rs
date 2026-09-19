//! #4 e2e first cut: bind → sign → accept → run → kill → failover → disk audit.

use gaia_memos::MemOs;
use gaia_orchestrator::{
    bind_plan_to_registry, persist_audit, Broker, IntentEngine, IntentSigner, LocalRunner,
    McpRegistry, TaskPlanner,
};

#[test]
fn e2e_stub_bind_sign_run_failover_writes_audit_file() {
    let mut mem = MemOs::new();
    let mut engine = IntentEngine::local_stub();
    let graph = engine
        .parse("research and summarize CARE", &mut mem)
        .unwrap();
    let signer = IntentSigner::generate();
    let id = engine.store(graph.clone(), &signer).unwrap();
    engine.verify_stored(id).unwrap();
    let signed = engine.get(id).unwrap().signed.clone();

    let mut plan = TaskPlanner::from_intent(&graph);
    bind_plan_to_registry(&mut plan, &McpRegistry::local()).unwrap();
    assert!(plan
        .nodes
        .iter()
        .all(|n| n.agent == "gaia-local-researcher"));
    plan.accept();

    let mut broker = Broker::new();
    let mut audit = gaia_orchestrator::TrustAudit::default();
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

    let path = std::env::temp_dir().join(format!("gaia-audit-{}.log", plan.id));
    persist_audit(&path, &audit).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.contains("node-failed-over:"));
    assert!(text.contains("node-completed:"));
    assert!(text.contains(&plan.intent_id.to_string()));
    let _ = std::fs::remove_file(&path);
}

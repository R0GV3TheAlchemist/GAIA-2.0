//! #4 placeholder trust bridge: deterministic test seam, explicitly not cryptography.

use gaia_memos::MemOs;
use gaia_orchestrator::{IntentEngine, PlaceholderSigner, TaskPlanner, TrustAudit};

fn intent() -> gaia_orchestrator::IntentGraph {
    IntentEngine::local_stub()
        .parse("research and summarize CARE", &MemOs::new())
        .unwrap()
}

#[test]
fn placeholder_signed_intent_verifies() {
    let graph = intent();
    let signer = PlaceholderSigner::new("local-test-key");
    let signed = signer.sign(&graph);
    assert_eq!(signed.algorithm, "PLACEHOLDER-NOT-CRYPTOGRAPHY");
    signer.verify(&signed).unwrap();
}

#[test]
fn tampered_or_missing_signature_is_rejected() {
    let graph = intent();
    let signer = PlaceholderSigner::new("local-test-key");
    let mut signed = signer.sign(&graph);
    signed.canonical_payload.push_str("tampered");
    assert!(signer.verify(&signed).is_err());
    signed.signature.clear();
    assert!(signer.verify(&signed).is_err());
}

#[test]
fn audit_binds_intent_plan_and_executor_append_only() {
    let graph = intent();
    let plan = TaskPlanner::from_intent(&graph);
    let mut audit = TrustAudit::default();
    audit.append(graph.id, Some(plan.id), Some("specialist-a"), "node-started");
    audit.append(graph.id, Some(plan.id), Some("specialist-a"), "node-completed");
    assert_eq!(audit.events().len(), 2);
    assert_eq!(audit.events()[0].sequence, 1);
    assert_eq!(audit.events()[1].sequence, 2);
    assert_eq!(audit.events()[0].intent_id, graph.id);
    assert_eq!(audit.events()[0].plan_id, Some(plan.id));
    assert_eq!(audit.events()[0].executor_id.as_deref(), Some("specialist-a"));
}

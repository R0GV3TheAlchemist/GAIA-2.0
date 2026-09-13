//! #4 kernel trust bridge: Ed25519 sign/verify and hash-chained audit.

use gaia_memos::MemOs;
use gaia_orchestrator::{IntentEngine, IntentSigner, TaskPlanner, TrustAudit};

fn intent() -> gaia_orchestrator::IntentGraph {
    IntentEngine::local_stub()
        .parse("research and summarize CARE", &MemOs::new())
        .unwrap()
}

#[test]
fn signed_intent_verifies_with_ed25519() {
    let graph = intent();
    let signer = IntentSigner::generate();
    let signed = signer.sign(&graph);
    assert_eq!(signed.algorithm, "ed25519");
    assert!(signed.issuer_did.starts_with("did:key:gaia:ed25519:"));
    signer.verify(&signed).unwrap();
}

#[test]
fn tampered_or_missing_signature_is_rejected() {
    let graph = intent();
    let signer = IntentSigner::generate();
    let mut signed = signer.sign(&graph);
    signed.canonical_payload.push_str("tampered");
    assert!(signer.verify(&signed).is_err());
    signed.signature.clear();
    assert!(signer.verify(&signed).is_err());
}

#[test]
fn foreign_key_cannot_verify_rewritten_signature() {
    let graph = intent();
    let a = IntentSigner::generate();
    let b = IntentSigner::generate();
    let mut signed = a.sign(&graph);
    signed.public_hex = b.principal().public_hex();
    assert!(IntentSigner::verify_detached(&signed).is_err());
}

#[test]
fn audit_binds_intent_plan_and_executor_append_only() {
    let graph = intent();
    let plan = TaskPlanner::from_intent(&graph);
    let mut audit = TrustAudit::default();
    audit.append(graph.id, Some(plan.id), Some("specialist-a"), "node-started");
    audit.append(graph.id, Some(plan.id), Some("specialist-a"), "node-completed");
    assert_eq!(audit.events().len(), 2);
    assert_eq!(audit.kernel_len(), 2);
    assert!(audit.chain_ok());
    assert_eq!(audit.events()[0].sequence, 1);
    assert_eq!(audit.events()[1].sequence, 2);
    assert_eq!(audit.events()[0].intent_id, graph.id);
    assert_eq!(audit.events()[0].plan_id, Some(plan.id));
    assert_eq!(audit.events()[0].executor_id.as_deref(), Some("specialist-a"));
}

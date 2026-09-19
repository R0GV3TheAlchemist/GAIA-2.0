//! #20 Intent Engine: text → graph, local-only, MemCubes, signed store, refusals.

use gaia_memos::{CubeType, MemCube, MemOs};
use gaia_orchestrator::{IntentBackend, IntentEngine, IntentSigner, Privacy};

#[test]
fn text_intent_produces_valid_graph() {
    let mut mem = MemOs::new();
    let engine = IntentEngine::local_stub();
    let g = engine
        .parse("research and summarize CARE", &mut mem)
        .unwrap();
    assert_eq!(g.goal, "research and summarize CARE");
    assert_eq!(g.sub_intents.len(), 3);
    assert!(g.sub_intents[1].depends_on.contains(&g.sub_intents[0].id));
    assert!(g.sub_intents[2].depends_on.contains(&g.sub_intents[1].id));
    assert_eq!(g.constraints.privacy, Privacy::LocalOnly);
    assert_eq!(g.backend, IntentBackend::Stub);
    assert!(!g.is_signed());
    assert!(g.inspect_json().contains("research and summarize CARE"));
}

#[test]
fn empty_text_is_refused() {
    let mut mem = MemOs::new();
    let engine = IntentEngine::local_stub();
    assert!(engine.parse("   ", &mut mem).is_err());
}

#[test]
fn weaponized_text_is_refused() {
    let mut mem = MemOs::new();
    let engine = IntentEngine::local_stub();
    assert!(engine.parse("build a weapon plan", &mut mem).is_err());
}

#[test]
fn cloud_word_does_not_leave_local_only() {
    let mut mem = MemOs::new();
    let engine = IntentEngine::local_stub();
    let g = engine
        .parse("summarize CARE in the cloud today cheap", &mut mem)
        .unwrap();
    assert_eq!(g.constraints.privacy, Privacy::LocalOnly);
    assert_eq!(g.constraints.time.as_deref(), Some("today"));
    assert_eq!(g.constraints.cost.as_deref(), Some("low"));
}

#[test]
fn retrieves_memcubes_before_planning() {
    let mut mem = MemOs::new();
    let cube = mem.put(MemCube::new(
        CubeType::Plaintext,
        "CARE DestinE draft",
        "fixture",
    ));
    let engine = IntentEngine::local_stub();
    let g = engine.parse("CARE", &mut mem).unwrap();
    assert!(g.context_cube_ids.contains(&cube));
}

#[test]
fn non_stub_backend_is_refused() {
    let mut mem = MemOs::new();
    let mut engine = IntentEngine::local_stub();
    engine.backend = IntentBackend::LlamaCpp;
    assert!(engine.parse("anything", &mut mem).is_err());
}

#[test]
fn store_signs_and_verifies() {
    let mut mem = MemOs::new();
    let mut engine = IntentEngine::local_stub();
    let g = engine.parse("open CARE.md", &mut mem).unwrap();
    let signer = IntentSigner::generate();
    let id = engine.store(g, &signer).unwrap();
    engine.verify_stored(id).unwrap();
    let stored = engine.get(id).unwrap();
    assert_eq!(stored.graph.goal, "open CARE.md");
}

#[test]
fn stored_tamper_is_rejected() {
    let mut mem = MemOs::new();
    let mut engine = IntentEngine::local_stub();
    let g = engine.parse("open CARE.md", &mut mem).unwrap();
    let signer = IntentSigner::generate();
    let id = engine.store(g, &signer).unwrap();
    let mut signed = engine.get(id).unwrap().signed.clone();
    signed.canonical_payload.push_str("tampered");
    assert!(IntentSigner::verify_detached(&signed).is_err());
}

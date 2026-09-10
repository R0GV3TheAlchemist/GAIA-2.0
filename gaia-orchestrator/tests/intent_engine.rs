//! #20 Intent Engine v0.1: text → graph, local-only, MemCubes, no cloud.

use gaia_memos::{CubeType, MemCube, MemOs};
use gaia_orchestrator::{IntentBackend, IntentEngine, Privacy};

#[test]
fn text_intent_produces_valid_graph() {
    let mem = MemOs::new();
    let engine = IntentEngine::local_stub();
    let g = engine.parse("research and summarize CARE", &mem).unwrap();
    assert_eq!(g.goal, "research and summarize CARE");
    assert_eq!(g.sub_intents.len(), 3);
    assert!(g.sub_intents[1].depends_on.contains(&g.sub_intents[0].id));
    assert!(g.sub_intents[2].depends_on.contains(&g.sub_intents[1].id));
    assert_eq!(g.constraints.privacy, Privacy::LocalOnly);
    assert_eq!(g.backend, IntentBackend::Stub);
    assert!(!g.is_signed(), "do not fake signatures");
}

#[test]
fn retrieves_memcubes_before_planning() {
    let mut mem = MemOs::new();
    let cube = mem.put(MemCube::new(CubeType::Plaintext, "CARE DestinE draft", "fixture"));
    let engine = IntentEngine::local_stub();
    let g = engine.parse("CARE", &mem).unwrap();
    assert!(g.context_cube_ids.contains(&cube));
}

#[test]
fn non_stub_backend_is_refused() {
    let mem = MemOs::new();
    let mut engine = IntentEngine::local_stub();
    engine.backend = IntentBackend::Ollama;
    assert!(engine.parse("anything", &mem).is_err());
    engine.backend = IntentBackend::LlamaCpp;
    assert!(engine.parse("anything", &mem).is_err());
}

#[test]
fn store_is_unsigned() {
    let mem = MemOs::new();
    let mut engine = IntentEngine::local_stub();
    let g = engine.parse("open CARE.md", &mem).unwrap();
    let id = engine.store(g);
    let stored = engine.get(id).unwrap();
    assert!(!stored.is_signed());
    assert_eq!(stored.goal, "open CARE.md");
}

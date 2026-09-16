use gaia_memos::MemOs;
use gaia_orchestrator::{IntentBackend, IntentEngine};

#[test]
fn llama_cpp_stays_unwired() {
    let mut mem = MemOs::new();
    let mut engine = IntentEngine::local_stub();
    engine.backend = IntentBackend::LlamaCpp;
    assert!(engine.parse("research CARE", &mut mem).unwrap_err().contains("not wired"));
}

#[test]
fn ollama_bad_port_fails_closed() {
    let mut mem = MemOs::new();
    let engine = IntentEngine::ollama_at("127.0.0.1:1", "llama3.2");
    let error = engine.parse("research CARE", &mut mem).unwrap_err();
    assert!(error.contains("unavailable") || error.contains("ollama"));
}

#[test]
fn live_ollama_proposal_has_retrieval_and_planner_minimum() {
    if std::env::var("GAIA_OLLAMA_LIVE").ok().as_deref() != Some("1") { return; }
    let mut mem = MemOs::new();
    let engine = IntentEngine::local_ollama();
    let graph = engine.parse("research and summarize CARE", &mut mem).expect("Ollama must be running for GAIA_OLLAMA_LIVE=1");
    assert_eq!(graph.backend, IntentBackend::Ollama);
    assert!((3..=4).contains(&graph.sub_intents.len()), "retrieve context plus two to three bounded model steps");
    assert!(graph.sub_intents[0].goal.starts_with("retrieve context:"));
    assert!(graph.sub_intents[0].depends_on.is_empty());
    assert!(graph.sub_intents[1].depends_on.contains(&graph.sub_intents[0].id));
}

use gaia_memos::MemOs;
use gaia_orchestrator::{IntentBackend, IntentEngine};

#[test]
fn llama_cpp_stays_unwired() {
    let mut mem = MemOs::new();
    let mut engine = IntentEngine::local_stub();
    engine.backend = IntentBackend::LlamaCpp;
    assert!(engine
        .parse("research CARE", &mut mem)
        .unwrap_err()
        .contains("not wired"));
}

#[test]
fn ollama_bad_port_fails_closed() {
    let mut mem = MemOs::new();
    let engine = IntentEngine::ollama_at("127.0.0.1:1", "llama3.2");
    let err = engine.parse("research CARE", &mut mem).unwrap_err();
    assert!(err.contains("unavailable") || err.contains("ollama"));
}

#[test]
fn live_ollama_marks_backend() {
    if std::env::var("GAIA_OLLAMA_LIVE").ok().as_deref() != Some("1") {
        return;
    }
    let mut mem = MemOs::new();
    let engine = IntentEngine::local_ollama();
    let g = engine
        .parse("research and summarize CARE", &mut mem)
        .expect("Ollama must be running for GAIA_OLLAMA_LIVE=1");
    assert_eq!(g.backend, IntentBackend::Ollama);
    assert_eq!(g.sub_intents.len(), 3);
}

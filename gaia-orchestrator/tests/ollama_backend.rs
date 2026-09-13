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
fn plan_json_requires_three_goals() {
    assert!(gaia_orchestrator_goals(r#"{"sub_intents":[{"goal":"a"},{"goal":"b"}]}"#).is_err());
    let goals = gaia_orchestrator_goals(
        r#"here {"sub_intents":[{"goal":"retrieve CARE"},{"goal":"read DestinE"},{"goal":"summarize"}]}"#, 
    )
    .unwrap();
    assert_eq!(goals.len(), 3);
    assert_eq!(goals[1], "read DestinE");
}

#[test]
fn plan_json_garbage_fails_closed() {
    assert!(gaia_orchestrator_goals("not json").is_err());
}

fn gaia_orchestrator_goals(raw: &str) -> Result<Vec<String>, String> {
    // Mirror crate::ollama::goals_from_plan_json without exporting in lib.rs.
    let start = raw.find('{').ok_or("ollama plan missing JSON object")?;
    let end = raw.rfind('}').ok_or("ollama plan missing JSON object")?;
    let obj = &raw[start..=end];
    let v: serde_json::Value = serde_json::from_str(obj).map_err(|e| e.to_string())?;
    let items = v
        .get("sub_intents")
        .and_then(|s| s.as_array())
        .ok_or("missing")?;
    let mut goals = Vec::new();
    for item in items {
        let goal = item
            .get("goal")
            .and_then(|g| g.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        if goal.is_empty() {
            return Err("empty".into());
        }
        goals.push(goal);
    }
    if goals.len() < 3 {
        return Err("need 3".into());
    }
    Ok(goals)
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
    assert!(g.sub_intents.len() >= 3);
    assert!(
        !g.sub_intents[0].goal.starts_with("retrieve context:"),
        "Ollama path must not use the stub template"
    );
}

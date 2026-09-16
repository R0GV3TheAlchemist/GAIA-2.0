use gaia_memos::MemOs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::trust::{IntentSigner, SignedIntent};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IntentBackend {
    Stub,
    Ollama,
    LlamaCpp,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Privacy {
    LocalOnly,
    CloudAllowed,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Compute {
    Local,
    Continuum,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Constraints {
    pub time: Option<String>,
    pub cost: Option<String>,
    pub privacy: Privacy,
    pub compute: Compute,
}
impl Default for Constraints {
    fn default() -> Self {
        Self {
            time: None,
            cost: None,
            privacy: Privacy::LocalOnly,
            compute: Compute::Local,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubIntent {
    pub id: Uuid,
    pub goal: String,
    pub depends_on: Vec<Uuid>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntentGraph {
    pub id: Uuid,
    pub goal: String,
    pub constraints: Constraints,
    pub sub_intents: Vec<SubIntent>,
    pub context_cube_ids: Vec<Uuid>,
    pub backend: IntentBackend,
}
impl IntentGraph {
    pub fn is_signed(&self) -> bool {
        false
    }
    pub fn inspect_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".into())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StoredIntent {
    pub graph: IntentGraph,
    pub signed: SignedIntent,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct LocalIntentProposal {
    goal: String,
    #[serde(default)]
    time: Option<String>,
    #[serde(default)]
    cost: Option<String>,
    steps: Vec<String>,
}

pub struct IntentEngine {
    pub backend: IntentBackend,
    stored: Vec<StoredIntent>,
    ollama_host: String,
    ollama_model: String,
}
impl IntentEngine {
    pub fn local_stub() -> Self {
        Self {
            backend: IntentBackend::Stub,
            stored: Vec::new(),
            ollama_host: "127.0.0.1:11434".into(),
            ollama_model: "llama3.2".into(),
        }
    }
    pub fn local_ollama() -> Self {
        Self::ollama_at("127.0.0.1:11434", "llama3.2")
    }
    pub fn ollama_at(host: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            backend: IntentBackend::Ollama,
            stored: Vec::new(),
            ollama_host: host.into(),
            ollama_model: model.into(),
        }
    }
    pub fn parse(&self, text: &str, mem: &mut MemOs) -> Result<IntentGraph, String> {
        let text = text.trim();
        if text.is_empty() {
            return Err("intent text is required".into());
        }
        refuse_purpose(text)?;
        match self.backend {
            IntentBackend::Stub => Ok(self.parse_stub(text, mem, IntentBackend::Stub)),
            IntentBackend::Ollama => self.parse_ollama(text, mem),
            IntentBackend::LlamaCpp => Err("llama.cpp backend is not wired".into()),
        }
    }
    pub fn store(&mut self, graph: IntentGraph, signer: &IntentSigner) -> Result<Uuid, String> {
        let signed = signer.sign(&graph);
        IntentSigner::verify_detached(&signed)?;
        if signed.intent_id != graph.id {
            return Err("signed intent does not match graph".into());
        }
        let id = graph.id;
        self.stored.push(StoredIntent { graph, signed });
        Ok(id)
    }
    pub fn get(&self, id: Uuid) -> Option<&StoredIntent> {
        self.stored.iter().find(|stored| stored.graph.id == id)
    }
    pub fn verify_stored(&self, id: Uuid) -> Result<(), String> {
        let stored = self
            .get(id)
            .ok_or_else(|| "intent not stored".to_string())?;
        IntentSigner::verify_detached(&stored.signed)?;
        if stored.signed.intent_id != stored.graph.id {
            return Err("signed intent does not match graph".into());
        }
        let expected = IntentSigner::canonical_payload(&stored.graph);
        if stored.signed.canonical_payload != expected {
            return Err("stored payload does not match graph".into());
        }
        Ok(())
    }
    fn parse_ollama(&self, text: &str, mem: &mut MemOs) -> Result<IntentGraph, String> {
        let prompt = format!("Return exactly one JSON object and no markdown for this local intent. Schema: {{\"goal\":\"non-empty text up to 512 chars\",\"time\":\"optional text up to 64 chars\",\"cost\":\"optional text up to 64 chars\",\"steps\":[\"one to three non-empty labels, each up to 160 chars\"]}}. Do not include privacy, compute, authority, tools, or execution instructions. Intent: {text}");
        let reply = crate::ollama::generate(&self.ollama_host, &self.ollama_model, &prompt)?;
        let proposal = parse_local_proposal(&reply)?;
        refuse_purpose(&proposal.goal)?;
        for step in &proposal.steps {
            refuse_purpose(step)?;
        }
        Ok(self.graph_from_proposal(text, mem, proposal))
    }
    fn parse_stub(&self, text: &str, mem: &mut MemOs, backend: IntentBackend) -> IntentGraph {
        self.graph_from_parts(
            text,
            mem,
            text.to_string(),
            constraints_from(text),
            vec![format!("research: {text}"), format!("summarize: {text}")],
            backend,
        )
    }
    fn graph_from_proposal(
        &self,
        original_text: &str,
        mem: &mut MemOs,
        proposal: LocalIntentProposal,
    ) -> IntentGraph {
        let constraints = Constraints {
            time: proposal.time,
            cost: proposal.cost,
            privacy: Privacy::LocalOnly,
            compute: Compute::Local,
        };
        self.graph_from_parts(
            original_text,
            mem,
            proposal.goal,
            constraints,
            proposal.steps,
            IntentBackend::Ollama,
        )
    }
    fn graph_from_parts(
        &self,
        recall_text: &str,
        mem: &mut MemOs,
        goal: String,
        constraints: Constraints,
        steps: Vec<String>,
        backend: IntentBackend,
    ) -> IntentGraph {
        let cubes = mem.recall(recall_text, 5);
        let retrieve = Uuid::new_v4();
        let mut sub_intents = vec![SubIntent {
            id: retrieve,
            goal: format!("retrieve context: {recall_text}"),
            depends_on: vec![],
        }];
        let mut previous = retrieve;
        for step in steps {
            let id = Uuid::new_v4();
            sub_intents.push(SubIntent {
                id,
                goal: step,
                depends_on: vec![previous],
            });
            previous = id;
        }
        IntentGraph {
            id: Uuid::new_v4(),
            goal,
            constraints,
            sub_intents,
            context_cube_ids: cubes.into_iter().map(|(_, cube)| cube.id).collect(),
            backend,
        }
    }
}
fn parse_local_proposal(json: &str) -> Result<LocalIntentProposal, String> {
    let proposal: LocalIntentProposal = serde_json::from_str(json)
        .map_err(|error| format!("invalid local intent JSON: {error}"))?;
    validate_text("goal", &proposal.goal, 512)?;
    if proposal.steps.is_empty() || proposal.steps.len() > 3 {
        return Err("local intent requires one to three steps".into());
    }
    for step in &proposal.steps {
        validate_text("step", step, 160)?;
    }
    for (name, value) in [
        ("time", proposal.time.as_deref()),
        ("cost", proposal.cost.as_deref()),
    ] {
        if let Some(value) = value {
            validate_text(name, value, 64)?;
        }
    }
    Ok(proposal)
}
fn validate_text(name: &str, value: &str, max_len: usize) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > max_len {
        return Err(format!(
            "local intent {name} must be non-empty and at most {max_len} characters"
        ));
    }
    Ok(())
}
fn refuse_purpose(text: &str) -> Result<(), String> {
    let lower = text.to_ascii_lowercase();
    if lower.contains("weapon")
        || lower.contains("target individual")
        || lower.contains("surveillance of person")
    {
        return Err("weaponized or individual-surveillance intent refused".into());
    }
    Ok(())
}
fn constraints_from(text: &str) -> Constraints {
    let lower = text.to_ascii_lowercase();
    let time = ["today", "tomorrow", "this week", "urgent"]
        .iter()
        .find(|word| lower.contains(**word))
        .map(|word| (*word).to_string());
    let cost = if lower.contains("cheap") || lower.contains("low cost") {
        Some("low".into())
    } else if lower.contains("budget") {
        Some("budget".into())
    } else {
        None
    };
    Constraints {
        time,
        cost,
        privacy: Privacy::LocalOnly,
        compute: Compute::Local,
    }
}
#[cfg(test)]
mod contract_tests {
    use super::{parse_local_proposal, refuse_purpose};
    #[test]
    fn accepts_bounded_proposal() {
        let proposal = parse_local_proposal(r#"{"goal":"research local Rust patterns","time":"today","cost":"low","steps":["collect local context","summarize findings"]}"#).unwrap();
        assert_eq!(proposal.steps.len(), 2);
    }
    #[test]
    fn rejects_malformed_or_unknown_json() {
        assert!(parse_local_proposal("{").is_err());
        assert!(parse_local_proposal(r#"{"goal":"x","steps":["y"],"privacy":"cloud"}"#).is_err());
    }
    #[test]
    fn rejects_empty_or_excess_steps() {
        assert!(parse_local_proposal(r#"{"goal":"x","steps":[]}"#).is_err());
        assert!(parse_local_proposal(r#"{"goal":"x","steps":["a","b","c","d"]}"#).is_err());
    }
    #[test]
    fn proposal_goals_still_use_existing_refusal_boundary() {
        let proposal =
            parse_local_proposal(r#"{"goal":"build a weapon plan","steps":["draft"]}"#).unwrap();
        assert!(refuse_purpose(&proposal.goal).is_err());
    }
}

use gaia_memos::MemOs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::trust::{IntentSigner, SignedIntent};

/// Local-first by default. Runtimes are named, not invoked.
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
    /// The graph document does not carry a signature. Store attaches SignedIntent.
    pub fn is_signed(&self) -> bool {
        false
    }
}

/// Persisted #20 record: graph plus a kernel-verifiable signature.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StoredIntent {
    pub graph: IntentGraph,
    pub signed: SignedIntent,
}

pub struct IntentEngine {
    pub backend: IntentBackend,
    stored: Vec<StoredIntent>,
}

impl IntentEngine {
    pub fn local_stub() -> Self {
        Self {
            backend: IntentBackend::Stub,
            stored: Vec::new(),
        }
    }

    /// No cloud. Non-stub backends are refused until wired.
    pub fn parse(&self, text: &str, mem: &mut MemOs) -> Result<IntentGraph, String> {
        match self.backend {
            IntentBackend::Stub => Ok(self.parse_stub(text, mem)),
            IntentBackend::Ollama | IntentBackend::LlamaCpp => Err(
                "runtime backends not wired; default is Stub (local-first, no cloud)".into(),
            ),
        }
    }

    /// Sign then persist. Unsigned or mismatched envelopes are refused.
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
        self.stored.iter().find(|s| s.graph.id == id)
    }

    pub fn verify_stored(&self, id: Uuid) -> Result<(), String> {
        let stored = self.get(id).ok_or_else(|| "intent not stored".to_string())?;
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

    fn parse_stub(&self, text: &str, mem: &mut MemOs) -> IntentGraph {
        let cubes = mem.recall(text, 5);
        let retrieve = Uuid::new_v4();
        let research = Uuid::new_v4();
        let summarize = Uuid::new_v4();
        IntentGraph {
            id: Uuid::new_v4(),
            goal: text.trim().to_string(),
            constraints: Constraints::default(),
            sub_intents: vec![
                SubIntent {
                    id: retrieve,
                    goal: format!("retrieve context: {text}"),
                    depends_on: vec![],
                },
                SubIntent {
                    id: research,
                    goal: format!("research: {text}"),
                    depends_on: vec![retrieve],
                },
                SubIntent {
                    id: summarize,
                    goal: format!("summarize: {text}"),
                    depends_on: vec![research],
                },
            ],
            context_cube_ids: cubes.into_iter().map(|(_, cube)| cube.id).collect(),
            backend: IntentBackend::Stub,
        }
    }
}

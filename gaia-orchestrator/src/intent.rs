use gaia_memos::MemOs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    /// Signing is #19 / #20 later. v0.1 does not pretend.
    pub fn is_signed(&self) -> bool {
        false
    }
}

pub struct IntentEngine {
    pub backend: IntentBackend,
    stored: Vec<IntentGraph>,
}

impl IntentEngine {
    pub fn local_stub() -> Self {
        Self {
            backend: IntentBackend::Stub,
            stored: Vec::new(),
        }
    }

    /// No cloud. Non-stub backends are refused until wired.
    pub fn parse(&self, text: &str, mem: &MemOs) -> Result<IntentGraph, String> {
        match self.backend {
            IntentBackend::Stub => Ok(self.parse_stub(text, mem)),
            IntentBackend::Ollama | IntentBackend::LlamaCpp => Err(
                "runtime backends not wired; default is Stub (local-first, no cloud)".into(),
            ),
        }
    }

    pub fn store(&mut self, graph: IntentGraph) -> Uuid {
        let id = graph.id;
        self.stored.push(graph);
        id
    }

    pub fn get(&self, id: Uuid) -> Option<&IntentGraph> {
        self.stored.iter().find(|g| g.id == id)
    }

    fn parse_stub(&self, text: &str, mem: &MemOs) -> IntentGraph {
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
            context_cube_ids: cubes.into_iter().map(|c| c.id).collect(),
            backend: IntentBackend::Stub,
        }
    }
}

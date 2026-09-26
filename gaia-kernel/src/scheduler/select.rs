//! [`AgentRegistry`] — Stage 4 (discovery) and Stage 5 (selection).

use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AgentHandle {
    pub id: String,
    pub capabilities: Vec<String>,
    pub load_score: f32,
}

impl AgentHandle {
    pub fn new(id: impl Into<String>, capabilities: Vec<String>) -> Self {
        Self {
            id: id.into(),
            capabilities,
            load_score: 0.0,
        }
    }
}

#[derive(Default, Clone)]
pub struct AgentRegistry {
    index: HashMap<String, Vec<AgentHandle>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, agent: AgentHandle) {
        for cap in &agent.capabilities.clone() {
            self.index.entry(cap.clone()).or_default().push(agent.clone());
        }
    }

    pub fn find(&self, capability: &str) -> Vec<AgentHandle> {
        self.index.get(capability).cloned().unwrap_or_default()
    }

    pub fn select(&self, candidates: Vec<AgentHandle>) -> AgentHandle {
        candidates
            .into_iter()
            .min_by(|a, b| {
                a.load_score
                    .partial_cmp(&b.load_score)
                    .unwrap_or(Ordering::Equal)
            })
            .expect("select called with empty candidate list")
    }
}

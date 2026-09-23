//! [`AgentRegistry`] — Stage 4 (discovery) and Stage 5 (selection).
//!
//! Agents register themselves with their capability tags.  The registry
//! returns all candidates for a required capability; the scheduler then
//! picks the best one based on load (stub: first registered wins).
//!
//! The full capability graph and load-aware selection are tracked in #721.

use std::cmp::Ordering;
use std::collections::HashMap;

/// A registered agent handle.
#[derive(Debug, Clone)]
pub struct AgentHandle {
    pub id:           String,
    /// Set of capability tags this agent can satisfy.
    pub capabilities: Vec<String>,
    /// Simulated current load score (lower = more available).
    pub load_score:   f32,
}

impl AgentHandle {
    pub fn new(id: impl Into<String>, capabilities: Vec<String>) -> Self {
        Self {
            id:           id.into(),
            capabilities,
            load_score:   0.0,
        }
    }
}

/// In-memory agent capability registry.
#[derive(Default)]
pub struct AgentRegistry {
    /// capability tag → list of agent handles
    index: HashMap<String, Vec<AgentHandle>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an agent for one or more capability tags.
    pub fn register(&mut self, agent: AgentHandle) {
        for cap in &agent.capabilities.clone() {
            self.index
                .entry(cap.clone())
                .or_default()
                .push(agent.clone());
        }
    }

    /// Stage 4 — return all agents that advertise this capability.
    pub fn find(&self, capability: &str) -> Vec<AgentHandle> {
        self.index
            .get(capability)
            .cloned()
            .unwrap_or_default()
    }

    /// Stage 5 — pick the lowest-load agent from the candidate list.
    /// Falls back to the first if all loads are equal.
    /// NaN load scores are treated as equal (Ordering::Equal) so a
    /// malformed score can never panic the scheduler.
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

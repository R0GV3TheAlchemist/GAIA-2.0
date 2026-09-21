//! [`Planner`] Stage 2 — decompose an [`Intent`] into [`SubGoal`]s.
//!
//! The decomposition strategy is intentionally simple in this first
//! implementation: one sub-goal per slot key in the intent.  The Orchestrator
//! issue (#721) will replace this with a real LLM-backed goal decomposer.

use crate::execution::{
    dag::{Task, TaskDAG},
    engine::Intent,
};

/// A single decomposed sub-goal derived from an Intent.
#[derive(Debug, Clone)]
pub struct SubGoal {
    pub capability: String,
    pub payload:    String,
}

/// Stateless planner — produces sub-goals and task DAGs from intents.
pub struct Planner;

impl Planner {
    pub fn new() -> Self {
        Self
    }

    /// Stage 2 — split intent into one sub-goal per slot.
    /// Falls back to a single sub-goal using `intent_type` when slots is empty.
    pub fn decompose(&self, intent: &Intent) -> Vec<SubGoal> {
        if intent.slots.is_empty() {
            return vec![SubGoal {
                capability: intent.intent_type.clone(),
                payload:    format!("intent:{}", intent.id),
            }];
        }
        intent
            .slots
            .iter()
            .map(|(k, v)| SubGoal {
                capability: k.clone(),
                payload:    v.clone(),
            })
            .collect()
    }

    /// Stage 3 — turn sub-goals into a flat (no-dependency) TaskDAG.
    /// Dependencies are added by the Orchestrator (#721) for complex plans.
    pub fn plan(&self, subgoals: &[SubGoal]) -> TaskDAG {
        let mut dag = TaskDAG::new();
        for sg in subgoals {
            dag.add_task(Task::new(sg.capability.clone(), sg.payload.clone()));
        }
        dag
    }
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}

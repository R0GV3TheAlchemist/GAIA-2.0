//! [`Replanner`] Stage 10 — adaptation / fallback logic.
//!
//! When a task fails, the Replanner attempts to produce a substitute task
//! with a degraded-but-acceptable capability.  The fallback map is seeded
//! with a small built-in table; the full capability graph (#721) will
//! replace this with a proper search over the CapabilityRegistry.

use crate::execution::dag::Task;

/// Stateless fallback planner.
pub struct Replanner {
    /// capability → fallback capability (degraded substitute)
    fallbacks: Vec<(&'static str, &'static str)>,
}

impl Replanner {
    pub fn new() -> Self {
        Self {
            fallbacks: vec![
                ("search",    "cached_search"),
                ("summarise", "extract"),
                ("plan",      "query"),
                ("alert",     "memory_write"),
            ],
        }
    }

    /// Return a fallback task if one exists for the failed capability, else `None`.
    pub fn fallback(&self, failed: &Task, _reason: &str) -> Option<Task> {
        self.fallbacks
            .iter()
            .find(|(cap, _)| *cap == failed.capability.as_str())
            .map(|(_, fallback_cap)| Task::new(*fallback_cap, failed.payload.clone()))
    }
}

impl Default for Replanner {
    fn default() -> Self {
        Self::new()
    }
}

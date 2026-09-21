//! [`Scheduler`] Stage 7 — resource quota allocation.
//!
//! Reserves CPU cores and memory for a task before execution begins.
//! This is a stub implementation: it always succeeds and returns the
//! task's self-declared resource hints.  Real quota enforcement against
//! a node resource map is tracked in #721 (Orchestrator).

use crate::execution::dag::Task;

/// Allocated resource quota for a single task execution.
#[derive(Debug, Clone)]
pub struct ResourceQuota {
    pub cpu_cores: u32,
    pub memory_mb: u32,
}

/// Stateless resource scheduler.
#[derive(Default)]
pub struct Scheduler;

impl Scheduler {
    pub fn new() -> Self {
        Self
    }

    /// Stage 7 — allocate resources for a task.
    /// Currently returns the task's self-declared hints unchanged.
    pub fn allocate(&self, task: &Task) -> ResourceQuota {
        ResourceQuota {
            cpu_cores: task.cpu_cores,
            memory_mb: task.memory_mb,
        }
    }
}

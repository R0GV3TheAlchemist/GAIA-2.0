//! [`TaskDAG`] — directed acyclic graph of execution tasks.
//!
//! The Planner produces a `TaskDAG`; the Engine iterates it in
//! topological order.  Tasks with no unmet dependencies are grouped into
//! a **parallel tier** and executed concurrently via `tokio::join`.
//!
//! # Invariants
//! - Every `task_id` in `edges` must also appear as a key in `tasks`.
//! - The graph must be acyclic; `topological_tiers` returns an error if a
//!   cycle is detected (should never happen from a well-formed Planner).

use std::collections::{HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Task ─────────────────────────────────────────────────────────────────────

/// A single unit of work inside the execution DAG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id:           Uuid,
    /// Human-readable capability tag (e.g. `"search"`, `"summarise"`).
    pub capability:   String,
    /// Serialised input payload for the agent.
    pub payload:      String,
    /// Resource hints for the Scheduler.
    pub cpu_cores:    u32,
    pub memory_mb:    u32,
}

impl Task {
    pub fn new(capability: impl Into<String>, payload: impl Into<String>) -> Self {
        Self {
            id:         Uuid::new_v4(),
            capability: capability.into(),
            payload:    payload.into(),
            cpu_cores:  1,
            memory_mb:  256,
        }
    }
}

// ── TaskDAG ──────────────────────────────────────────────────────────────────

/// Directed acyclic graph of [`Task`]s produced by the Planner.
#[derive(Debug, Default)]
pub struct TaskDAG {
    /// All tasks keyed by their UUID.
    pub tasks: HashMap<Uuid, Task>,
    /// `edges[a]` = set of tasks that must complete *before* `a` may start.
    pub edges: HashMap<Uuid, HashSet<Uuid>>,
}

impl TaskDAG {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a task with no dependencies.
    pub fn add_task(&mut self, task: Task) {
        self.edges.entry(task.id).or_default();
        self.tasks.insert(task.id, task);
    }

    /// Declare that `dependent` must wait for `prerequisite`.
    pub fn add_edge(&mut self, prerequisite: Uuid, dependent: Uuid) {
        self.edges.entry(dependent).or_default().insert(prerequisite);
    }

    /// Return tasks grouped into parallel tiers (Kahn's algorithm).
    ///
    /// Tier 0 = tasks with no dependencies (run first, in parallel).
    /// Tier N = tasks whose prerequisites are all in tiers < N.
    ///
    /// Returns `Err` if the graph contains a cycle.
    pub fn topological_tiers(&self) -> Result<Vec<Vec<Task>>, String> {
        let mut in_degree: HashMap<Uuid, usize> = self
            .tasks
            .keys()
            .map(|id| (*id, self.edges.get(id).map(|s| s.len()).unwrap_or(0)))
            .collect();

        // Build reverse adjacency: for each node, who depends on it?
        let mut dependents: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        for (node, prereqs) in &self.edges {
            for prereq in prereqs {
                dependents.entry(*prereq).or_default().push(*node);
            }
        }

        let mut queue: VecDeque<Uuid> = in_degree
            .iter()
            .filter(|(_, &d)| d == 0)
            .map(|(id, _)| *id)
            .collect();

        let mut tiers: Vec<Vec<Task>> = Vec::new();
        let mut visited = 0usize;

        while !queue.is_empty() {
            let tier_size = queue.len();
            let mut tier = Vec::with_capacity(tier_size);

            for _ in 0..tier_size {
                // Safety: the outer while-guard and the fixed `tier_size`
                // iteration count guarantee the queue is non-empty here.
                // Use expect() to document the invariant rather than silently
                // swallowing a logic bug with unwrap_or.
                let id = queue
                    .pop_front()
                    .expect("queue must be non-empty inside tier_size loop");
                tier.push(self.tasks[&id].clone());
                visited += 1;

                if let Some(deps) = dependents.get(&id) {
                    for dep in deps {
                        let deg = in_degree
                            .get_mut(dep)
                            .expect("every dep must be present in in_degree");
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(*dep);
                        }
                    }
                }
            }

            tiers.push(tier);
        }

        if visited != self.tasks.len() {
            return Err("cycle detected in TaskDAG".into());
        }

        Ok(tiers)
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    /// Replace a single failed task node with a substitute and return a new DAG
    /// that preserves all unaffected nodes and edges.
    pub fn replace_node(&self, failed_id: Uuid, replacement: Task) -> Self {
        let mut dag = Self::new();
        for (id, task) in &self.tasks {
            if *id == failed_id {
                dag.add_task(replacement.clone());
            } else {
                dag.add_task(task.clone());
            }
        }
        let new_id = replacement.id;
        for (node, prereqs) in &self.edges {
            let mapped_node = if *node == failed_id { new_id } else { *node };
            for prereq in prereqs {
                let mapped_prereq = if *prereq == failed_id { new_id } else { *prereq };
                dag.add_edge(mapped_prereq, mapped_node);
            }
        }
        dag
    }
}

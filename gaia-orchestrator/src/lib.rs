//! L4 Cognitive orchestration (Phase 2 / #4).
//! Part 1 (#20): Intent Engine stub.
//! Part 2 (#21): Task Planner DAG, inspect-before-run, fallback.

mod dag;
mod intent;

pub use dag::{DagNode, Executor, Plan, ResourceEstimate, RunReport, TaskPlanner};
pub use intent::{
    Compute, Constraints, IntentBackend, IntentEngine, IntentGraph, Privacy, SubIntent,
};

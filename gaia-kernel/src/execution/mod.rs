//! Execution sub-system: DAG, error taxonomy, metrics, and the 12-stage engine.

pub mod dag;
pub mod engine;
pub mod error;
pub mod metrics;

pub use engine::{ExecutionEngine, ExecutionResult, Intent, IntentSignature, Outcome, TaskResult};
pub use dag::{Task, TaskDAG};
pub use error::ExecutionError;

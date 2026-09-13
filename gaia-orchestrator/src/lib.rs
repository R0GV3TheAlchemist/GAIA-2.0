//! L4 Cognitive orchestration (Phase 2 / #4).
//! Part 1 (#20): Intent Engine stub; signed store.
//! Part 2 (#21): Task Planner DAG, inspect-before-run, retries, fallback, persist.
//! Part 3 (#22): Pull-based broker, failover, reconcile, carbon timetable.
//! Part 4 (#23): MCP JSON-RPC session, AIP registry, unsigned reject.
//! Part 5 (#4): Kernel Ed25519 intent signatures and hash-chained audit.
//! Part 6 (#4): Approved local DAG run, failover, lifecycle audit.
//! Part 7 (#4): Local Ollama generate; fail closed; llama.cpp still refused.

mod broker;
mod dag;
mod intent;
mod mcp;
mod ollama;
mod run;
mod trust;

pub use broker::{Broker, CarbonTimetable, Metrics, ReconcileReport, Worker};
pub use dag::{
    DagNode, Executor, NodeAttempt, Plan, ResourceEstimate, RunReport, TaskPlanner,
};
pub use intent::{
    Compute, Constraints, IntentBackend, IntentEngine, IntentGraph, Privacy, StoredIntent, SubIntent,
};
pub use mcp::{
    AipManifest, DiscoveryStub, JsonRpcRequest, JsonRpcResponse, McpMessage, McpRegistry,
    McpResource, McpTool,
};
pub use run::{LocalRun, LocalRunner};
pub use trust::{
    verify_tagged_signature, AuditEvent, IntentSigner, SignedIntent, TrustAudit,
};

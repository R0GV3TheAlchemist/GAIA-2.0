//! L4 cognitive orchestration.
//! Part 1 (#20): Local-first intent parse and inspectable graph.
//! Part 2 (#21): Task Planner DAG, inspect-before-run, fallback.
//! Part 3 (#22): Pull-based broker, failover, local metrics.
//! Part 4 (#23): MCP stub, AIP registry, unsigned reject.
//! Part 5 (#4): Kernel Ed25519 intent signatures and hash-chained audit.
//! Part 6 (#4): Approved local DAG run, failover, lifecycle audit.

mod audit_disk;
mod broker;
mod dag;
mod github_source;
mod github_source_config;
mod intent;
mod mcp;
mod mcp_stdio;
mod ollama;
mod run;
mod select;
mod trace;
mod trust;

pub use audit_disk::persist_audit;
pub use broker::{Broker, Metrics, Worker};
pub use dag::{DagNode, Executor, Plan, ResourceEstimate, RunReport, TaskPlanner};
pub use github_source::{GitHubSource, GitHubSourceError};
pub use github_source_config::GitHubSourceConfig;
pub use intent::{
    Compute, Constraints, IntentBackend, IntentEngine, IntentGraph, Privacy, SubIntent,
};
pub use mcp::{AipManifest, DiscoveryStub, JsonRpcRequest, JsonRpcResponse, McpMessage, McpRegistry, McpTool};
pub use mcp_stdio::{decode_line, encode_line, persist_session};
pub use run::{LocalRun, LocalRunner};
pub use select::{bind_plan_to_registry, pick_agent};
pub use trace::{
    control_plane_unavailable_event, gap_lock_event, permit_execution, ExecutionGate, GateState,
    InMemoryGate, InMemorySink, NoopSink, RunPermit, SafeTraceMeta, TraceEvent, TraceEventKind,
    TraceEventSink,
};
pub use trust::{verify_tagged_signature, AuditEvent, IntentSigner, SignedIntent, TrustAudit};

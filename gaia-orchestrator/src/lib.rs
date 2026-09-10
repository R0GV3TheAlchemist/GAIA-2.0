//! L4 Cognitive orchestration (Phase 2 / #4).
//! Part 1 (#20): Intent Engine stub.
//! Part 2 (#21): Task Planner DAG, inspect-before-run, fallback.
//! Part 3 (#22): Pull-based broker, failover, local metrics.
//! Part 4 (#23): MCP stub, AIP registry, unsigned reject.
//! Part 5 (#4): Non-cryptographic placeholder trust bridge; replace with #19 API.

mod broker;
mod dag;
mod intent;
mod mcp;
mod trust;

pub use broker::{Broker, Metrics, Worker};
pub use dag::{DagNode, Executor, Plan, ResourceEstimate, RunReport, TaskPlanner};
pub use intent::{
    Compute, Constraints, IntentBackend, IntentEngine, IntentGraph, Privacy, SubIntent,
};
pub use mcp::{AipManifest, DiscoveryStub, McpMessage, McpRegistry, McpTool};
pub use trust::{AuditEvent, PlaceholderSigner, SignedIntent, TrustAudit};

//! L4 Cognitive orchestration (Phase 2 / #4).

mod audit_disk;
mod broker;
mod dag;
mod github_source;
mod intent;
mod mcp;
mod mcp_stdio;
mod ollama;
mod run;
mod select;
mod trust;

pub use audit_disk::persist_audit;
pub use broker::{Broker, CarbonTimetable, Metrics, ReconcileReport, Worker};
pub use dag::{DagNode, Executor, NodeAttempt, Plan, ResourceEstimate, RunReport, TaskPlanner};
pub use github_source::{FakeGitHubSourceProvider, GitHubSourcePolicy, SourceAuditInput, SourceCacheKey, SourceOperation};
pub use intent::{Compute, Constraints, IntentBackend, IntentEngine, IntentGraph, Privacy, StoredIntent, SubIntent};
pub use mcp::{AipManifest, DiscoveryStub, JsonRpcRequest, JsonRpcResponse, McpMessage, McpRegistry, McpResource, McpTool};
pub use mcp_stdio::{decode_line, encode_line, persist_session};
pub use run::{LocalRun, LocalRunner};
pub use select::{bind_plan_to_registry, pick_agent};
pub use trust::{verify_tagged_signature, AuditEvent, IntentSigner, SignedIntent, TrustAudit};

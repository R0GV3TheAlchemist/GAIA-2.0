//! L4 Cognitive orchestration (Phase 2 / #4).

mod audit_disk;
mod broker;
mod cognitive;
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
pub use broker::{Broker, CarbonTimetable, Metrics, ReconcileReport, Worker};
pub use cognitive::{
    build_plan, AdaptationEngine, AgentTeam, AllocationPlan, CapabilityMatcher,
    ConstraintResolver, ConstraintViolation, EacnEntry, ExecutionPlanner, FailureRecovery,
    GoalDecomposer, PlanError, PlanGenerator, PlanOutcome, PlanValidator, RecoveryAction,
    RecoveryStrategy,
};
pub use dag::{DagNode, Executor, NodeAttempt, Plan, ResourceEstimate, RunReport, TaskPlanner};
pub use github_source::{
    FakeGitHubSourceProvider, GitHubSourcePolicy, SourceAuditInput, SourceCacheKey, SourceOperation,
};
pub use github_source_config::load_github_source_policy;
pub use intent::{
    Compute, Constraints, IntentBackend, IntentEngine, IntentGraph, Privacy, StoredIntent,
    SubIntent,
};
pub use mcp::{
    AipManifest, DiscoveryStub, JsonRpcRequest, JsonRpcResponse, McpMessage, McpRegistry,
    McpResource, McpTool,
};
pub use mcp_stdio::{decode_line, encode_line, persist_session};
pub use run::{LocalRun, LocalRunner};
pub use select::{bind_plan_to_registry, pick_agent};
pub use trace::{
    control_plane_unavailable_event, gap_lock_event, permit_execution, ExecutionGate, GateState,
    InMemoryGate, InMemorySink, NoopSink, RunPermit, SafeTraceMeta, TraceEvent, TraceEventKind,
    TraceEventSink,
};
pub use trust::{verify_tagged_signature, AuditEvent, IntentSigner, SignedIntent, TrustAudit};

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // ExecutionGate (InMemoryGate) — control-plane boundary enforcement
    // -------------------------------------------------------------------------

    /// A freshly constructed InMemoryGate must default to Open.
    /// The gate is the canonical boundary between the orchestrator and the
    /// control plane. Defaulting to Open means work proceeds unless the
    /// control plane explicitly signals unavailability.
    #[test]
    fn execution_gate_default_state_is_open() {
        let gate = InMemoryGate::new();
        assert_eq!(
            gate.state(),
            GateState::Open,
            "InMemoryGate must default to Open"
        );
    }

    /// After a control_plane_unavailable_event the gate must transition to Closed.
    /// Any call to permit_execution on a Closed gate must return Err, never Ok.
    #[test]
    fn execution_gate_blocks_when_control_plane_unavailable() {
        let mut gate = InMemoryGate::new();
        let sink = InMemorySink::new();
        control_plane_unavailable_event(&mut gate, &sink);
        assert_eq!(
            gate.state(),
            GateState::Closed,
            "gate must be Closed after control_plane_unavailable_event"
        );
        let result = permit_execution(&gate);
        assert!(
            result.is_err(),
            "permit_execution on a Closed gate must return Err"
        );
    }

    // -------------------------------------------------------------------------
    // Broker — work-queue invariants
    // -------------------------------------------------------------------------

    /// A new Broker has no workers and no pending work.
    #[test]
    fn broker_new_is_empty() {
        let broker = Broker::new();
        assert_eq!(broker.worker_count(), 0, "new Broker must have zero workers");
        assert_eq!(broker.pending_count(), 0, "new Broker must have zero pending items");
    }

    // -------------------------------------------------------------------------
    // TrustAudit — signed intent roundtrip
    // -------------------------------------------------------------------------

    /// Signing an intent and immediately verifying it must succeed.
    /// This is the minimal roundtrip that proves the signing key and
    /// verification path are wired together correctly.
    #[test]
    fn trust_audit_signed_intent_roundtrip() {
        let signer = IntentSigner::generate();
        let payload = b"intent:query|user:did:gaia:test|ts:1000";
        let signed  = signer.sign(payload);
        assert!(
            TrustAudit::verify(&signed, payload).is_ok(),
            "valid signed intent must verify successfully"
        );
    }

    // -------------------------------------------------------------------------
    // McpRegistry — tool registration and lookup
    // -------------------------------------------------------------------------

    /// A tool registered in McpRegistry must be retrievable by name.
    #[test]
    fn mcp_registry_register_and_lookup() {
        let mut registry = McpRegistry::new();
        let tool = McpTool {
            name:        "search".into(),
            description: "semantic search over GAIA canon".into(),
            input_schema: serde_json::json!({}),
        };
        registry.register_tool(tool);
        let found = registry.get_tool("search");
        assert!(found.is_some(), "registered tool must be retrievable by name");
        assert_eq!(found.unwrap().name, "search");
    }
}

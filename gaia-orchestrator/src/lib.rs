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

    /// A freshly constructed InMemoryGate must default to the Clear state.
    /// The gate is the canonical boundary between the orchestrator and the
    /// control plane. Defaulting to Clear means work proceeds unless the
    /// control plane explicitly signals unavailability or a gap lock.
    /// Note: the constructor is InMemoryGate::clear(), not ::new().
    #[test]
    fn execution_gate_default_state_is_open() {
        let gate = InMemoryGate::clear();
        assert_eq!(
            gate.state(),
            GateState::Clear,
            "InMemoryGate::clear() must have GateState::Clear"
        );
    }

    /// An InMemoryGate in the ControlPlaneUnavailable state must produce a
    /// RunPermit::Deny when deployed=true. This is the primary fail-close
    /// invariant: a closed gate must never permit execution in production.
    ///
    /// API notes:
    /// - control_plane_unavailable_event() returns a TraceEvent; it does not
    ///   mutate the gate. Construct the gate via InMemoryGate::unavailable().
    /// - permit_execution(gate, deployed) takes two arguments and returns
    ///   RunPermit (not Result); check via pattern match.
    #[test]
    fn execution_gate_blocks_when_control_plane_unavailable() {
        let gate = InMemoryGate::unavailable();
        assert_eq!(
            gate.state(),
            GateState::ControlPlaneUnavailable,
            "InMemoryGate::unavailable() must have GateState::ControlPlaneUnavailable"
        );
        let permit = permit_execution(&gate, true); // deployed = true → fail-close
        assert!(
            matches!(permit, RunPermit::Deny { .. }),
            "permit_execution on an unavailable gate with deployed=true must return Deny"
        );
    }

    // -------------------------------------------------------------------------
    // Broker — work-queue invariants
    // -------------------------------------------------------------------------

    /// A new Broker pre-populates one manager and two specialist workers with
    /// an empty queue. This verifies the initial topology has not regressed.
    #[test]
    fn broker_new_is_empty() {
        let broker = Broker::new();
        assert_eq!(
            broker.workers.len(), 3,
            "new Broker must have 3 workers (1 manager + 2 specialists)"
        );
        assert!(
            broker.queue.is_empty(),
            "new Broker must have an empty work queue"
        );
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

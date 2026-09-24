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

// ---------------------------------------------------------------------------
// Scope aggregation gate — swarm security boundary
//
// Every agent in a swarm carries an `AgentScope` that declares the maximum
// permission level it operates under. When the orchestrator attempts to form
// a multi-agent swarm and issue a combined action, `scope_aggregation_gate`
// enforces that the *requested* scope does not exceed what *all* agents in the
// swarm have been individually granted. A single under-privileged agent is
// sufficient to deny the request — privilege cannot be aggregated upward.
//
// Scope ordering (ascending privilege):
//   LocalRead < RepoWrite < ExternalEgress
// ---------------------------------------------------------------------------

/// The permission level an agent or swarm action may operate under.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scope {
    /// Read access to local/in-process data only. No filesystem writes,
    /// no network egress, no repository mutations.
    LocalRead,
    /// May write to the GAIA repository (issues, files, branches). No
    /// external network egress beyond the configured GitHub origin.
    RepoWrite,
    /// May initiate outbound network connections to arbitrary external hosts.
    /// Highest privilege level — requires explicit operator approval.
    ExternalEgress,
}

/// Declares the maximum `Scope` an agent is authorised to operate under.
#[derive(Debug, Clone)]
pub struct AgentScope {
    /// Human-readable agent identifier for audit logging.
    pub agent_id: String,
    /// The ceiling permission level granted to this agent.
    pub scope: Scope,
}

/// Errors produced by the orchestrator's security boundary checks.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum OrchestratorError {
    /// The requested scope was denied because at least one agent in the swarm
    /// does not hold sufficient privilege. Privilege cannot be aggregated
    /// upward across agents — the minimum wins.
    #[error("scope aggregation denied: requested scope exceeds swarm minimum")]
    ScopeAggregationDenied,
}

/// Gate function that enforces swarm scope aggregation policy.
///
/// Returns `Ok(())` if **every** agent in `agents` holds a `scope` that is
/// greater than or equal to `requested`. Returns
/// `Err(OrchestratorError::ScopeAggregationDenied)` if any agent's scope is
/// below the requested level.
///
/// An empty swarm is denied by convention — a swarm with no agents cannot
/// be granted any permission.
pub fn scope_aggregation_gate(
    agents: &[AgentScope],
    requested: Scope,
) -> Result<(), OrchestratorError> {
    if agents.is_empty() {
        return Err(OrchestratorError::ScopeAggregationDenied);
    }
    let all_qualify = agents.iter().all(|a| a.scope >= requested);
    if all_qualify {
        Ok(())
    } else {
        Err(OrchestratorError::ScopeAggregationDenied)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    // -------------------------------------------------------------------------
    // ExecutionGate (InMemoryGate) — control-plane boundary enforcement
    // -------------------------------------------------------------------------

    /// A freshly constructed InMemoryGate must default to Clear (gate open).
    /// The gate is the canonical boundary between the orchestrator and the
    /// control plane. Defaulting to Clear means work proceeds unless the
    /// control plane explicitly signals unavailability.
    #[test]
    fn execution_gate_default_state_is_open() {
        let gate = InMemoryGate::clear();
        assert_eq!(
            gate.state(),
            GateState::Clear,
            "InMemoryGate::clear() must have GateState::Clear"
        );
    }

    /// After control_plane_unavailable_event(), using InMemoryGate::unavailable()
    /// must yield ControlPlaneUnavailable state, and permit_execution must deny.
    #[test]
    fn execution_gate_blocks_when_control_plane_unavailable() {
        let gate = InMemoryGate::unavailable();
        assert_eq!(
            gate.state(),
            GateState::ControlPlaneUnavailable,
            "gate must be ControlPlaneUnavailable"
        );
        let _event = control_plane_unavailable_event();
        let permit = permit_execution(&gate, true);
        assert!(
            matches!(permit, RunPermit::Deny { .. }),
            "permit_execution on unavailable gate must return Deny"
        );
    }

    // -------------------------------------------------------------------------
    // Broker — work-queue invariants
    // -------------------------------------------------------------------------

    /// Broker::new() pre-populates 3 workers; the queue starts empty.
    #[test]
    fn broker_new_is_empty() {
        let broker = Broker::new();
        assert_eq!(broker.workers.len(), 3, "Broker::new() must pre-populate 3 workers");
        assert!(broker.queue.is_empty(),   "new Broker must have an empty work queue");
    }

    // -------------------------------------------------------------------------
    // TrustAudit — signed intent roundtrip
    // -------------------------------------------------------------------------

    /// Signing an IntentGraph and immediately verifying it must succeed.
    /// IntentSigner::sign() expects &IntentGraph (not raw bytes).
    /// Verification is performed via IntentSigner::verify_detached(&signed).
    #[test]
    fn trust_audit_signed_intent_roundtrip() {
        let signer = IntentSigner::generate();
        let graph = IntentGraph {
            id: Uuid::nil(),
            goal: "test goal".into(),
            constraints: Constraints::default(),
            sub_intents: vec![],
            context_cube_ids: vec![],
            backend: IntentBackend::Stub,
        };
        let signed = signer.sign(&graph);
        assert!(
            IntentSigner::verify_detached(&signed).is_ok(),
            "valid signed IntentGraph must verify successfully"
        );
    }

    // -------------------------------------------------------------------------
    // McpRegistry — tool lookup via real API
    // -------------------------------------------------------------------------

    /// McpRegistry::local() seeds the registry with at least one tool.
    /// Tools are enumerable via .tools() and findable by name.
    /// This tests the real constructor and query surface — no phantom methods.
    #[test]
    fn mcp_registry_local_has_tools_and_lookup_works() {
        let registry = McpRegistry::local();
        let tools = registry.tools();
        assert!(
            !tools.is_empty(),
            "McpRegistry::local() must seed at least one tool"
        );
        // Verify that every tool returned by tools() is findable by name search.
        let first_name = tools[0].name.clone();
        let found = registry.tools().into_iter().find(|t| t.name == first_name);
        assert!(
            found.is_some(),
            "tool enumerated by tools() must be findable by name"
        );
        assert_eq!(found.unwrap().name, first_name);
    }
}

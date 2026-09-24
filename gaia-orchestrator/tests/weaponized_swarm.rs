//! Adversarial test: Weaponized Swarm — cross-agent scope aggregation (#958).
//!
//! Forbidden-role bind: docs/agents/FORBIDDEN-ROLE-TEST-BIND.md
//! Canon: C30 (no silent failures), moral-architecture P1 (power without domination)
//! Parent epic: #901
//!
//! A weaponized swarm occurs when multiple specialist agents each hold a
//! sub-threshold scope, but their combined aggregated scope would exceed the
//! operator-granted ceiling. The orchestrator must detect and block aggregation.

use gaia_orchestrator::*;

/// Two specialists each with read scope — combined, they must not be permitted
/// to aggregate into a write scope.
#[test]
fn cross_agent_scope_aggregation_is_blocked() {
    let agents = vec![
        AgentScope {
            agent_id: "spec-a".into(),
            scope: Scope::LocalRead,
        },
        AgentScope {
            agent_id: "spec-b".into(),
            scope: Scope::LocalRead,
        },
    ];
    let requested = Scope::RepoWrite;
    assert_eq!(
        scope_aggregation_gate(&agents, requested),
        Err(OrchestratorError::ScopeAggregationDenied),
        "two read-scoped specialists must not aggregate to write scope"
    );
}

/// A single specialist requesting a scope it was not individually granted is denied.
#[test]
fn single_agent_scope_escalation_is_blocked() {
    let agents = vec![AgentScope {
        agent_id: "spec-a".into(),
        scope: Scope::LocalRead,
    }];
    let requested = Scope::ExternalEgress;
    assert_eq!(
        scope_aggregation_gate(&agents, requested),
        Err(OrchestratorError::ScopeAggregationDenied),
        "a single read-scoped specialist must not escalate to external egress"
    );
}

/// Specialists within their individually-granted aggregate scope are permitted.
#[test]
fn within_granted_aggregate_scope_passes() {
    let agents = vec![
        AgentScope {
            agent_id: "spec-a".into(),
            scope: Scope::LocalRead,
        },
        AgentScope {
            agent_id: "spec-b".into(),
            scope: Scope::LocalRead,
        },
    ];
    let requested = Scope::LocalRead;
    assert!(
        scope_aggregation_gate(&agents, requested).is_ok(),
        "two read-scoped specialists requesting read scope must be permitted"
    );
}

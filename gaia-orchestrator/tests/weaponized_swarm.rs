//! Adversarial test: Weaponized Swarm — cross-agent scope aggregation (#958).
//!
//! Forbidden-role bind: docs/agents/FORBIDDEN-ROLE-TEST-BIND.md
//! Canon: C30, moral-architecture P1
//! Parent epic: #901

use gaia_orchestrator::*;

#[test]
fn cross_agent_scope_aggregation_is_blocked() {
    let agents = vec![
        AgentScope { agent_id: "spec-a".into(), scope: Scope::LocalRead },
        AgentScope { agent_id: "spec-b".into(), scope: Scope::LocalRead },
    ];
    assert_eq!(
        scope_aggregation_gate(&agents, Scope::RepoWrite),
        Err(OrchestratorError::ScopeAggregationDenied)
    );
}

#[test]
fn single_agent_scope_escalation_is_blocked() {
    let agents = vec![
        AgentScope { agent_id: "spec-a".into(), scope: Scope::LocalRead },
    ];
    assert_eq!(
        scope_aggregation_gate(&agents, Scope::ExternalEgress),
        Err(OrchestratorError::ScopeAggregationDenied)
    );
}

#[test]
fn within_granted_aggregate_scope_passes() {
    let agents = vec![
        AgentScope { agent_id: "spec-a".into(), scope: Scope::LocalRead },
        AgentScope { agent_id: "spec-b".into(), scope: Scope::LocalRead },
    ];
    assert!(scope_aggregation_gate(&agents, Scope::LocalRead).is_ok());
}

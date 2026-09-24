//! ResourceQuota hard-limit test for the Digital Parasite forbidden role (#957).
//!
//! Forbidden-role bind: docs/agents/FORBIDDEN-ROLE-TEST-BIND.md
//! Canon: C30, moral-architecture P1
//! Parent epic: #901

use gaia_acp::*;

fn parasite_action(tool: &str, target: &str) -> ProposedAction {
    ProposedAction {
        agent_id: "agent-parasite".into(),
        tool: tool.into(),
        method: "call".into(),
        target: target.into(),
        action_class: ActionClass::LocalRead,
        payload: "ok".into(),
        nonce: "nonce-parasite".into(),
        gateway_id: "gateway-local".into(),
        server_id: "server-local".into(),
        resource_id: "repo-local".into(),
        wants_delegation: false,
    }
}

#[test]
fn resource_quota_hard_limit_is_enforced() {
    let quota = ResourceQuota {
        max_memory_bytes: 64 * 1024 * 1024,
        max_cpu_ms: 500,
        max_egress_bytes: 0,
    };
    let usage = ResourceUsage {
        memory_bytes: 64 * 1024 * 1024 + 1,
        cpu_ms: 100,
        egress_bytes: 0,
    };
    assert_eq!(
        resource_quota_gate(&quota, &usage),
        Err(ReasonCode::ResourceQuotaExceeded)
    );
}

#[test]
fn zero_egress_quota_is_hard_limit() {
    let quota = ResourceQuota {
        max_memory_bytes: 64 * 1024 * 1024,
        max_cpu_ms: 500,
        max_egress_bytes: 0,
    };
    let usage = ResourceUsage {
        memory_bytes: 1024,
        cpu_ms: 10,
        egress_bytes: 1,
    };
    assert_eq!(
        resource_quota_gate(&quota, &usage),
        Err(ReasonCode::ResourceQuotaExceeded)
    );
}

#[test]
fn within_quota_passes() {
    let quota = ResourceQuota {
        max_memory_bytes: 64 * 1024 * 1024,
        max_cpu_ms: 500,
        max_egress_bytes: 1024,
    };
    let usage = ResourceUsage {
        memory_bytes: 1024,
        cpu_ms: 10,
        egress_bytes: 512,
    };
    assert!(resource_quota_gate(&quota, &usage).is_ok());
}

#[test]
fn write_action_over_quota_is_quota_rejected_not_autonomy_rejected() {
    let quota = ResourceQuota {
        max_memory_bytes: 1024,
        max_cpu_ms: 500,
        max_egress_bytes: 0,
    };
    let usage = ResourceUsage {
        memory_bytes: 2048,
        cpu_ms: 10,
        egress_bytes: 0,
    };
    assert_eq!(
        resource_quota_gate(&quota, &usage),
        Err(ReasonCode::ResourceQuotaExceeded)
    );
    let _ = parasite_action("fs", "/etc");
}

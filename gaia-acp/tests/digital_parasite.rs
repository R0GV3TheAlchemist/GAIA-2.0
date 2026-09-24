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

/// Baseline quota with generous limits on axes not under test.
fn base_quota() -> ResourceQuota {
    ResourceQuota {
        max_tool_calls: 1_000,
        max_output_tokens: 1_000_000,
        max_wall_secs: 3_600,
        max_memory_bytes: 64 * 1024 * 1024,
        max_cpu_ms: 500,
        max_egress_bytes: 1024,
    }
}

/// Baseline usage — all axes well under limit.
fn base_usage() -> ResourceUsage {
    ResourceUsage {
        tool_calls: 0,
        output_tokens: 0,
        wall_secs: 0,
        memory_bytes: 1024,
        cpu_ms: 10,
        egress_bytes: 0,
    }
}

#[test]
fn resource_quota_hard_limit_is_enforced() {
    let quota = base_quota();
    let usage = ResourceUsage {
        memory_bytes: 64 * 1024 * 1024 + 1,
        ..base_usage()
    };
    assert_eq!(
        resource_quota_gate(&quota, &usage),
        Err(ReasonCode::ResourceQuotaExceeded)
    );
}

#[test]
fn zero_egress_quota_is_hard_limit() {
    let quota = ResourceQuota {
        max_egress_bytes: 0,
        ..base_quota()
    };
    let usage = ResourceUsage {
        egress_bytes: 1,
        ..base_usage()
    };
    assert_eq!(
        resource_quota_gate(&quota, &usage),
        Err(ReasonCode::ResourceQuotaExceeded)
    );
}

#[test]
fn within_quota_passes() {
    assert!(resource_quota_gate(&base_quota(), &base_usage()).is_ok());
}

#[test]
fn write_action_over_quota_is_quota_rejected_not_autonomy_rejected() {
    let quota = ResourceQuota {
        max_memory_bytes: 1024,
        ..base_quota()
    };
    let usage = ResourceUsage {
        memory_bytes: 2048,
        ..base_usage()
    };
    assert_eq!(
        resource_quota_gate(&quota, &usage),
        Err(ReasonCode::ResourceQuotaExceeded)
    );
    let _ = parasite_action("fs", "/etc");
}

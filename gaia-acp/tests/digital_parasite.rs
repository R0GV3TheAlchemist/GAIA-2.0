//! ResourceQuota hard-limit test for the Digital Parasite forbidden role (#957).
//!
//! Forbidden-role bind: docs/agents/FORBIDDEN-ROLE-TEST-BIND.md
//! Canon: C30 (no silent failures), moral-architecture P1 (power without domination)
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

/// A Digital Parasite attempts to exceed its declared ResourceQuota.
/// The gate must return `Err(ReasonCode::ResourceQuotaExceeded)` — never silently permit.
#[test]
fn resource_quota_hard_limit_is_enforced() {
    let quota = ResourceQuota {
        max_memory_bytes: 64 * 1024 * 1024, // 64 MiB ceiling
        max_cpu_ms: 500,
        max_egress_bytes: 0, // zero egress permitted
    };
    let usage = ResourceUsage {
        memory_bytes: 64 * 1024 * 1024 + 1, // one byte over
        cpu_ms: 100,
        egress_bytes: 0,
    };
    assert_eq!(
        resource_quota_gate(&quota, &usage),
        Err(ReasonCode::ResourceQuotaExceeded),
        "memory one byte over quota must be hard-rejected"
    );
}

/// Egress is zero-permitted for this role; any byte of egress is a violation.
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
        egress_bytes: 1, // one byte of egress
    };
    assert_eq!(
        resource_quota_gate(&quota, &usage),
        Err(ReasonCode::ResourceQuotaExceeded),
        "one byte of egress against a zero-egress quota must be hard-rejected"
    );
}

/// A well-behaved agent within all limits must pass the quota gate.
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
    assert!(
        resource_quota_gate(&quota, &usage).is_ok(),
        "usage within all limits must pass the quota gate"
    );
}

/// A Digital Parasite attempting a write action while over memory quota is doubly rejected.
/// The quota gate is checked before the autonomy gate; quota is the harder outer shell.
#[test]
fn write_action_over_quota_is_quota_rejected_not_autonomy_rejected() {
    let quota = ResourceQuota {
        max_memory_bytes: 1024,
        max_cpu_ms: 500,
        max_egress_bytes: 0,
    };
    let usage = ResourceUsage {
        memory_bytes: 2048, // over
        cpu_ms: 10,
        egress_bytes: 0,
    };
    // Quota gate fires first — reason code is ResourceQuotaExceeded, not AutonomyCap.
    assert_eq!(
        resource_quota_gate(&quota, &usage),
        Err(ReasonCode::ResourceQuotaExceeded)
    );
}

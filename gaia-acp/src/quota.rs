//! Resource quota gate — Digital Parasite adversarial scenario (#957).
//! Guards against agents that consume unbounded memory, CPU, or egress.
//! No I/O. Pure deterministic logic.

use serde::{Deserialize, Serialize};

use crate::types::ReasonCode;

/// Hard limits set by the operator or policy engine for a single agent session.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceQuota {
    /// Maximum resident memory in bytes.
    pub max_memory_bytes: u64,
    /// Maximum CPU time in milliseconds.
    pub max_cpu_ms: u64,
    /// Maximum network egress in bytes. Set to 0 to prohibit all outbound traffic.
    pub max_egress_bytes: u64,
}

/// Live snapshot of what an agent has consumed so far.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceUsage {
    pub memory_bytes: u64,
    pub cpu_ms: u64,
    pub egress_bytes: u64,
}

/// Returns `Ok(())` when usage is within quota on all axes.
/// Returns `Err(ReasonCode::ResourceQuotaExceeded)` the moment any axis is over limit.
/// Takes both arguments by reference so callers need not move the values.
pub fn resource_quota_gate(quota: &ResourceQuota, usage: &ResourceUsage) -> Result<(), ReasonCode> {
    if usage.memory_bytes > quota.max_memory_bytes
        || usage.cpu_ms > quota.max_cpu_ms
        || usage.egress_bytes > quota.max_egress_bytes
    {
        return Err(ReasonCode::ResourceQuotaExceeded);
    }
    Ok(())
}

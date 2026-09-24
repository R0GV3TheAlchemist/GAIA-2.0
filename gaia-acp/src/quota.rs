//! Resource quota gate (#digital-parasite adversarial scenario).
//! Guards against agents that consume unbounded compute, memory, or API call budgets.

use serde::{Deserialize, Serialize};

use crate::types::ReasonCode;

/// Hard limits set by the operator or policy engine for a single agent session.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceQuota {
    /// Maximum number of tool invocations allowed in this session.
    pub max_tool_calls: u64,
    /// Maximum total output tokens the agent may generate.
    pub max_output_tokens: u64,
    /// Maximum wall-clock time in seconds the session may run.
    pub max_wall_secs: u64,
    /// Maximum resident memory in bytes.
    pub max_memory_bytes: u64,
    /// Maximum CPU time in milliseconds.
    pub max_cpu_ms: u64,
    /// Maximum network egress in bytes. Set to 0 to prohibit all outbound traffic.
    pub max_egress_bytes: u64,
}

/// Snapshot of what an agent has actually consumed so far.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceUsage {
    pub tool_calls: u64,
    pub output_tokens: u64,
    pub wall_secs: u64,
    pub memory_bytes: u64,
    pub cpu_ms: u64,
    pub egress_bytes: u64,
}

/// Returns `Ok(())` if usage is within quota on every axis.
/// Returns `Err(ReasonCode::ResourceQuotaExceeded)` as soon as any axis is over.
/// Takes both arguments by reference so callers need not move the values.
pub fn resource_quota_gate(quota: &ResourceQuota, usage: &ResourceUsage) -> Result<(), ReasonCode> {
    if usage.tool_calls > quota.max_tool_calls
        || usage.output_tokens > quota.max_output_tokens
        || usage.wall_secs > quota.max_wall_secs
        || usage.memory_bytes > quota.max_memory_bytes
        || usage.cpu_ms > quota.max_cpu_ms
        || usage.egress_bytes > quota.max_egress_bytes
    {
        return Err(ReasonCode::ResourceQuotaExceeded);
    }
    Ok(())
}

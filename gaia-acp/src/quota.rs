//! Resource quota gate — Digital Parasite adversarial scenario (#957).
//! Guards against agents that consume unbounded compute, token, or time budgets.
//! No I/O. Pure deterministic logic.

use serde::{Deserialize, Serialize};

use crate::types::ReasonCode;

/// Hard limits set by the operator or policy engine for a single agent session.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceQuota {
    /// Maximum tool invocations allowed in this session.
    pub max_tool_calls: u64,
    /// Maximum total output tokens the agent may generate.
    pub max_output_tokens: u64,
    /// Maximum wall-clock seconds the session may run.
    pub max_wall_secs: u64,
}

/// Live snapshot of what an agent has consumed so far.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceUsage {
    pub tool_calls: u64,
    pub output_tokens: u64,
    pub wall_secs: u64,
}

/// Returns `Ok(())` when usage is within quota on all axes.
/// Returns `Err(ReasonCode::ResourceQuotaExceeded)` the moment any axis is over limit.
pub fn resource_quota_gate(quota: ResourceQuota, usage: ResourceUsage) -> Result<(), ReasonCode> {
    if usage.tool_calls > quota.max_tool_calls
        || usage.output_tokens > quota.max_output_tokens
        || usage.wall_secs > quota.max_wall_secs
    {
        return Err(ReasonCode::ResourceQuotaExceeded);
    }
    Ok(())
}

//! Capability registration and monopoly-risk gate (#958 — monopoly-architect adversarial scenario).
//!
//! Enforces moral-architecture P1 (power without domination) and C30 (no silent failures):
//! every registered capability must declare at least two independent resolution paths so
//! no single agent can become a mandatory chokepoint.

use crate::host::KernelError;

/// A request to register a named capability with the kernel.
///
/// The kernel uses this to enforce dual-path redundancy: a capability that
/// can only be resolved through a single node is a monopoly risk and will
/// be rejected before it is admitted to the broker.
#[derive(Debug, Clone)]
pub struct CapabilityRegistration {
    /// Stable, human-readable identifier for this capability (e.g. `"cap-data-retrieval"`).
    pub id: String,
    /// The independent resolution paths (node IDs, service endpoints, etc.) that can
    /// satisfy this capability. Must contain at least two entries to pass the gate.
    pub resolution_paths: Vec<String>,
}

/// Returns `Ok(())` when the registration is safe to admit.
///
/// Rejects with [`KernelError::MonopolyRisk`] when fewer than two independent
/// resolution paths are declared, because a single-path or zero-path capability
/// gives one agent exclusive control over that capability.
pub fn redundancy_gate(reg: &CapabilityRegistration) -> Result<(), KernelError> {
    if reg.resolution_paths.len() < 2 {
        return Err(KernelError::MonopolyRisk);
    }
    Ok(())
}

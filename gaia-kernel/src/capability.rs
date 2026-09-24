//! Capability redundancy gate — Monopoly Architect adversarial scenario (#958).
//! A capability registered with fewer than two resolution paths creates a
//! single point of control (monopoly risk) and must be rejected.
//! No I/O. Pure deterministic logic.

use serde::{Deserialize, Serialize};

use crate::host::KernelError;

/// A capability the kernel is asked to register, along with every node path
/// that can resolve it. At least two distinct paths are required.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapabilityRegistration {
    /// Unique capability identifier (e.g. `"cap-data-retrieval"`).
    pub id: String,
    /// Ordered list of node / service identifiers that can fulfil this
    /// capability. Must contain at least two entries.
    pub resolution_paths: Vec<String>,
}

/// Returns `Ok(())` when the capability has at least two resolution paths.
/// Returns `Err(KernelError::MonopolyRisk)` when there is zero or one path,
/// because a single-path capability gives one node exclusive control.
pub fn redundancy_gate(cap: &CapabilityRegistration) -> Result<(), KernelError> {
    if cap.resolution_paths.len() < 2 {
        return Err(KernelError::MonopolyRisk);
    }
    Ok(())
}

//! Adversarial test: Monopoly Architect — dual-path redundancy (#958).
//!
//! Forbidden-role bind: docs/agents/FORBIDDEN-ROLE-TEST-BIND.md
//! Canon: C30 (no silent failures), moral-architecture P1 (power without domination)
//! Parent epic: #901
//!
//! A monopoly architect eliminates redundant paths so all traffic must route
//! through a single controlled chokepoint. The kernel must enforce that any
//! registered capability has at least two independent resolution paths.

use gaia_kernel::*;

/// A capability registered with only one resolution path is a single point of
/// control — the kernel must reject it with `MonopolyRisk`.
#[test]
fn single_path_capability_is_rejected() {
    let cap = CapabilityRegistration {
        id: "cap-data-retrieval".into(),
        resolution_paths: vec!["node-a".into()], // only one path
    };
    assert_eq!(
        redundancy_gate(&cap),
        Err(KernelError::MonopolyRisk),
        "a capability with a single resolution path must be rejected as a monopoly risk"
    );
}

/// A capability with zero resolution paths must also be rejected.
#[test]
fn zero_path_capability_is_rejected() {
    let cap = CapabilityRegistration {
        id: "cap-orphan".into(),
        resolution_paths: vec![],
    };
    assert_eq!(
        redundancy_gate(&cap),
        Err(KernelError::MonopolyRisk),
        "a capability with zero resolution paths must be rejected"
    );
}

/// A capability with two or more independent resolution paths passes.
#[test]
fn dual_path_capability_passes() {
    let cap = CapabilityRegistration {
        id: "cap-data-retrieval".into(),
        resolution_paths: vec!["node-a".into(), "node-b".into()],
    };
    assert!(
        redundancy_gate(&cap).is_ok(),
        "a capability with two independent resolution paths must pass the redundancy gate"
    );
}

/// Three or more paths also pass — redundancy requirement is a minimum of two, not exactly two.
#[test]
fn triple_path_capability_also_passes() {
    let cap = CapabilityRegistration {
        id: "cap-data-retrieval".into(),
        resolution_paths: vec!["node-a".into(), "node-b".into(), "node-c".into()],
    };
    assert!(
        redundancy_gate(&cap).is_ok(),
        "three resolution paths must satisfy the dual-path minimum"
    );
}

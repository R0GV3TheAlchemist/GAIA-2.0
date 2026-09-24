//! Adversarial test: Monopoly Architect — dual-path redundancy (#958).
//!
//! Forbidden-role bind: docs/agents/FORBIDDEN-ROLE-TEST-BIND.md
//! Canon: C30, moral-architecture P1
//! Parent epic: #901

use gaia_kernel::*;

#[test]
fn single_path_capability_is_rejected() {
    let cap = CapabilityRegistration {
        id: "cap-data-retrieval".into(),
        resolution_paths: vec!["node-a".into()],
    };
    assert_eq!(redundancy_gate(&cap), Err(KernelError::MonopolyRisk));
}

#[test]
fn zero_path_capability_is_rejected() {
    let cap = CapabilityRegistration {
        id: "cap-orphan".into(),
        resolution_paths: vec![],
    };
    assert_eq!(redundancy_gate(&cap), Err(KernelError::MonopolyRisk));
}

#[test]
fn dual_path_capability_passes() {
    let cap = CapabilityRegistration {
        id: "cap-data-retrieval".into(),
        resolution_paths: vec!["node-a".into(), "node-b".into()],
    };
    assert!(redundancy_gate(&cap).is_ok());
}

#[test]
fn triple_path_capability_also_passes() {
    let cap = CapabilityRegistration {
        id: "cap-data-retrieval".into(),
        resolution_paths: vec!["node-a".into(), "node-b".into(), "node-c".into()],
    };
    assert!(redundancy_gate(&cap).is_ok());
}

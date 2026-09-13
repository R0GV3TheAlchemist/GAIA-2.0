//! #31 two local instances, residency lock, egress audit.

use gaia_kernel::{Federation, FederationError};

#[test]
fn two_local_instances_complete_a_joint_task() {
    let mut net = Federation::new();
    let austin = net.spawn("US-TX");
    let dallas = net.spawn("US-TX");
    net.peer(&austin, &dallas).unwrap();
    let cube = Federation::cube("cube-1", "US-TX", "local notes");
    let task = net
        .joint_task(&austin, &dallas, "research CARE", &cube)
        .unwrap();
    assert_eq!(task.from, austin.did);
    assert_eq!(task.to, dallas.did);
    assert!(task.redacted);
    assert!(net.audit.prove("egress.allowed", "cube-1").is_some());
}

#[test]
fn policy_forbids_a_cube_from_leaving_its_region() {
    let mut net = Federation::new();
    let texas = net.spawn("US-TX");
    let eu = net.spawn("EU-DE");
    net.peer(&texas, &eu).unwrap();
    let cube = Federation::cube("vault-9", "US-TX", "resident memory");
    let err = net
        .joint_task(&texas, &eu, "share memory", &cube)
        .unwrap_err();
    assert_eq!(
        err,
        FederationError::ResidencyDenied {
            cube: "vault-9".into(),
            region: "US-TX".into(),
        }
    );
}

#[test]
fn egress_attempts_appear_in_the_audit_log() {
    let mut net = Federation::new();
    let texas = net.spawn("US-TX");
    let eu = net.spawn("EU-DE");
    net.peer(&texas, &eu).unwrap();
    let cube = Federation::cube("vault-9", "US-TX", "resident memory");
    let _ = net.joint_task(&texas, &eu, "share memory", &cube);
    assert!(net.egress_denied("vault-9"));
    assert!(net.audit.prove("egress.denied", "vault-9").is_some());
    assert!(net.audit.chain_ok());
}

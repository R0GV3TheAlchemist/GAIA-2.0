//! #543 listed shelf. Existing APIs only.

use gaia_aikd::{aikd_v1_tagged, audit_report, knows_everything};

#[test]
fn v1_gate_and_non_omniscience() {
    assert!(!aikd_v1_tagged());
    assert!(!knows_everything());
    assert_eq!(audit_report().len(), 4);
    assert!(audit_report().iter().any(|l| l.contains("cannot-know")));
    assert!(audit_report().iter().any(|l| l.contains("unmeasured")));
}

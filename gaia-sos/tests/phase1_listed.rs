//! #467 listed shelf. Existing APIs only.

use gaia_sos::{kernel_inference, live_slurm, second_kernel, sfs_v1, sos_v1_tagged};

#[test]
fn sfs_is_fuse_label_and_kernel_inference_is_optional_tcb() {
    assert_eq!(sfs_v1(), "fuse");
    assert_eq!(kernel_inference(), "optional-tcb-risk");
}

#[test]
fn phase1_does_not_ship_a_kernel_or_live_slurm() {
    assert!(!second_kernel());
    assert!(!live_slurm());
    assert!(!sos_v1_tagged());
}

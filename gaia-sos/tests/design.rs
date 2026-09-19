use gaia_sos::{
    discover, five_nines_claimed, formal_verify_done, god_coordinator, kernel_inference,
    revoke_mode, sfs_v1, sos_v1_tagged, t0_kernel_kloc, threats,
};

#[test]
fn contracts_are_honest() {
    assert_eq!(revoke_mode(), "strongly-consistent");
    assert!(t0_kernel_kloc() < 500);
    assert_eq!(sfs_v1(), "fuse");
    assert_eq!(kernel_inference(), "optional-tcb-risk");
    discover(true).unwrap();
    assert!(god_coordinator().is_err());
    assert!(threats().contains(&"prompt-injection"));
    assert!(!formal_verify_done());
    assert!(!five_nines_claimed());
    assert!(!sos_v1_tagged());
}

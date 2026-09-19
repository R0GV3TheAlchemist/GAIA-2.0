//! #471 listed shelf. Existing APIs only.

use gaia_sos::{five_nines_claimed, formal_verify_done, sos_v1_tagged, threats};

#[test]
fn threat_model_is_named_and_v1_is_a_closed_gate() {
    assert_eq!(threats().len(), 5);
    assert!(threats().contains(&"prompt-injection"));
    assert!(threats().contains(&"supply-chain"));
    assert!(!formal_verify_done());
    assert!(!sos_v1_tagged());
    assert!(!five_nines_claimed());
}

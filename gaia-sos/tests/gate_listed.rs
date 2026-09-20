//! #529 listed shelf. Existing APIs only.

use gaia_sos::{five_nines_claimed, formal_verify_done, sos_v1_tagged, threats};

#[test]
fn v1_gate_stays_closed() {
    assert!(!sos_v1_tagged());
    assert!(!five_nines_claimed());
    assert!(!formal_verify_done());
    assert!(threats().contains(&"prompt-injection"));
}

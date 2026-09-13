use gaia_sos::{five_nines_claimed, rsi, second_kernel, sentience, sos_v1_tagged};

#[test]
fn no_second_kernel_rsi_sentience_or_fake_nines() {
    assert!(!second_kernel());
    assert!(!rsi());
    assert!(!sentience());
    assert!(!five_nines_claimed());
    assert!(!sos_v1_tagged());
}

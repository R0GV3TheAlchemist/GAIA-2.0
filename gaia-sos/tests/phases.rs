use gaia_sos::{five_nines_claimed, flutter_ui, live_llm, live_marketplace, phase_closed, profiles, sos_v1_tagged};

#[test]
fn phase_0_1_closed_later_phases_not_products() {
    assert!(phase_closed(2));
    assert!(phase_closed(3));
    assert!(!phase_closed(4));
    assert!(!live_llm());
    assert!(!live_marketplace());
    assert!(!flutter_ui());
    assert_eq!(profiles().len(), 4);
    assert!(!sos_v1_tagged());
    assert!(!five_nines_claimed());
}

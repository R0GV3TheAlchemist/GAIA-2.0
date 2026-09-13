use gaia_sos::{five_nines_claimed, flutter_ui, inbound_ports_required, live_fuse, live_llm, live_marketplace, live_ollama, live_qdrant, phase_closed, profiles, sos_v1_tagged};

#[test]
fn phase_0_1_closed_later_not_products() {
    assert!(phase_closed(2) && phase_closed(3));
    assert!(!phase_closed(4));
    assert!(!live_llm());
    assert!(!live_marketplace());
    assert!(!flutter_ui());
    assert_eq!(profiles().len(), 4);
    assert!(!live_fuse());
    assert!(!live_qdrant());
    assert!(!live_ollama());
    assert!(!inbound_ports_required());
    assert!(!sos_v1_tagged());
    assert!(!five_nines_claimed());
}

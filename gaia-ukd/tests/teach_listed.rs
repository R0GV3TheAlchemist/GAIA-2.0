//! #567 listed shelf. Existing APIs only.

use gaia_ukd::{
    climate_boundary_links, offline_pack, sm2, teach_me, ukd_v1_tagged, GaianMode, KnowledgeState,
};

#[test]
fn teach_uses_state() {
    let mut s = KnowledgeState::local();
    assert!(teach_me("x", &s).unwrap().contains("new"));
    s.mark_learned("x").unwrap();
    assert!(teach_me("x", &s).unwrap().contains("known"));
}

#[test]
fn climate_tek_only_if_granted() {
    let closed = climate_boundary_links(false).unwrap();
    assert!(closed.iter().any(|l| l.contains("climatology")));
    assert!(!closed.iter().any(|l| l.contains("traditional-knowledge")));
    let open = climate_boundary_links(true).unwrap();
    assert!(open.iter().any(|l| l.contains("traditional-knowledge")));
    assert_eq!(sm2("x", 5).interval_days, 6);
    assert!(offline_pack(GaianMode::Learn).contains("offline"));
    assert!(!ukd_v1_tagged());
}

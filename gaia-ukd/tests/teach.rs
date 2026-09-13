use gaia_ukd::{climate_boundary_links, teach_me, KnowledgeState};

#[test]
fn teach_uses_state_and_tek_link_needs_grant() {
    let mut state = KnowledgeState::local();
    state.known.push("climate".into());
    assert!(teach_me("climate", &state).unwrap().contains("known"));
    let open = climate_boundary_links(false).unwrap();
    assert!(open.contains(&"ukd:earth-systems:climatology"));
    assert!(!open.iter().any(|l| l.contains("traditional-knowledge")));
    let granted = climate_boundary_links(true).unwrap();
    assert!(granted.iter().any(|l| l.contains("traditional-knowledge")));
}

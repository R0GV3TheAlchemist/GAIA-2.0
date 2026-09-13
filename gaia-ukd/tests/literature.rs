use gaia_ukd::{present, KnowledgeLevel, KnowledgeState, LicensedNode};

#[test]
fn paper_and_skill_have_licenses_and_state_updates() {
    assert!(!LicensedNode::paper().license.is_empty());
    assert!(!LicensedNode::skill().license.is_empty());
    let sprout = present("algebra", KnowledgeLevel::Sprout);
    let master = present("algebra", KnowledgeLevel::Master);
    assert_ne!(sprout, master);
    let mut state = KnowledgeState::local();
    state.learning.push("linear-algebra".into());
    state.mark_learned("linear-algebra").unwrap();
    assert!(state.known.contains(&"linear-algebra".into()));
    assert!(!state.sync);
}

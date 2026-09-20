//! #563 listed shelf. Existing APIs only.

use gaia_ukd::{present, ukd_v1_tagged, KnowledgeLevel, KnowledgeState, LicensedNode};

#[test]
fn paper_and_skill_cite_licenses() {
    assert!(!LicensedNode::paper().license.is_empty());
    assert!(!LicensedNode::skill().license.is_empty());
}

#[test]
fn two_levels_and_local_state() {
    let a = present("linear-algebra", KnowledgeLevel::Sprout);
    let b = present("linear-algebra", KnowledgeLevel::Master);
    assert_ne!(a, b);
    let mut s = KnowledgeState::local();
    assert!(!s.sync);
    s.mark_learned("linear-algebra").unwrap();
    assert!(s.known.iter().any(|k| k == "linear-algebra"));
    assert!(!ukd_v1_tagged());
}

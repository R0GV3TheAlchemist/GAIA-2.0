//! #505 listed shelf. Existing APIs only.

use gaia_skills::{active_listening, SkillError, SkillGraph};

#[test]
fn seed_is_unmapped_and_queryable_by_missing_ids() {
    let g = SkillGraph::seed();
    assert!(!g.unmapped().is_empty());
    assert_eq!(g.by_esco("missing").unwrap_err(), SkillError::UnknownSkill);
    assert_eq!(g.by_onet("missing").unwrap_err(), SkillError::UnknownSkill);
    let n = active_listening();
    assert!(n.esco.is_none());
    assert!(n.onet.is_none());
    assert_eq!(n.id, "skill:active-listening");
}

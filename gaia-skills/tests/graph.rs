use gaia_skills::SkillGraph;

#[test]
fn unmapped_skills_exist_without_esco() {
    let graph = SkillGraph::seed();
    assert!(!graph.unmapped().is_empty());
    assert!(graph.by_esco("missing").is_err());
}

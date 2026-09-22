//! Tests for #622 AI Skills Database.

use gaia_aikd::{
    all_skills, assess_skill, learning_path, search_skills, skill_by_id, skill_gap_analysis,
    skills_for_domain, skills_for_tool,
};

#[test]
fn all_registered_skills_have_schema_fields() {
    let skills = all_skills();
    assert_eq!(skills.len(), 18);
    for skill in skills {
        assert!(!skill.skill_id.is_empty());
        assert!(!skill.domain.is_empty());
        assert!(!skill.description.is_empty());
        assert!(!skill.benchmark.is_empty());
        assert!(!skill.model.is_empty());
        assert!(!skill.tool_implementation.is_empty());
        assert!(skill.performance_score <= 100);
    }
}

#[test]
fn lookup_and_domain_filters_work() {
    let debug = skill_by_id("debugging").expect("debugging skill must exist");
    assert_eq!(debug.benchmark, "SWE-bench-Verified");

    let code_skills = skills_for_domain("code");
    assert!(code_skills.iter().any(|s| s.skill_id == "algorithm-design"));
    assert!(code_skills.iter().any(|s| s.skill_id == "debugging"));

    let sandbox_skills = skills_for_tool("sandbox");
    assert!(sandbox_skills.iter().any(|s| s.skill_id == "code-execution"));
    assert!(sandbox_skills.iter().any(|s| s.skill_id == "debugging"));
}

#[test]
fn search_returns_top_three_relevant_skills() {
    let results = search_skills("web browsing and search with pages");
    assert!(!results.is_empty());
    assert!(results.len() <= 3);
    assert_eq!(results[0].skill_id, "web-search");

    let code_results = search_skills("debug repository issue and fix bug");
    assert!(!code_results.is_empty());
    assert!(code_results.iter().any(|s| s.skill_id == "debugging"));
}

#[test]
fn assessment_api_assigns_levels() {
    let debugging = assess_skill("debugging").expect("assessment exists");
    assert_eq!(debugging.score, 95);
    assert_eq!(debugging.level, "expert");

    let gui = assess_skill("gui-navigation").expect("assessment exists");
    assert_eq!(gui.level, "weak");
}

#[test]
fn gap_analysis_identifies_missing_skills() {
    let gap = skill_gap_analysis("browse the web and summarize pages", &["tool-selection"]);
    assert_eq!(gap.goal, "browse the web and summarize pages");
    assert!(gap.present.iter().any(|s| s.skill_id == "tool-selection"));
    assert!(gap.missing.iter().any(|s| s.skill_id == "web-search"));
}

#[test]
fn learning_pathways_exist() {
    let debugging = learning_path("debugging").expect("learning path exists");
    assert!(debugging.prerequisites.contains(&"algorithm-design"));
    assert!(debugging.benchmarks_to_watch.contains(&"SWE-bench-Verified"));

    let generic = learning_path("document-parsing").expect("generic learning path exists");
    assert!(generic.tool_components.contains(&"ocr-wasm"));
}

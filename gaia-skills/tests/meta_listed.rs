//! #489 listed shelf. Existing APIs only.

use gaia_skills::{
    bessi_domains, digcomp_areas, global_profile_dump, hidden_profile_api, skills_v1_tagged,
    tek_skill, wef_top10, SkillCard, SkillError,
};

#[test]
fn overlay_tag_lists_exist() {
    assert_eq!(wef_top10().len(), 10);
    assert_eq!(digcomp_areas().len(), 5);
    assert_eq!(bessi_domains().len(), 5);
}

#[test]
fn local_default_no_v1_no_tek_scrape_no_child_rank() {
    assert!(!skills_v1_tagged());
    assert!(!global_profile_dump());
    assert!(!hidden_profile_api());
    assert_eq!(tek_skill(false).unwrap_err(), SkillError::NoGrant);
    let child = SkillCard {
        published: true,
        age_years: 12,
        skill: "demo".into(),
    };
    assert_eq!(child.rank_in_marketplace().unwrap_err(), SkillError::ChildRank);
}

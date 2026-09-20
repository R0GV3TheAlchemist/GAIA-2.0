//! #501 listed shelf. Existing APIs only.

use gaia_skills::{global_profile_dump, skills_v1_tagged, tek_skill, SkillCard, SkillError};

#[test]
fn unpublished_and_ungranted_tek_stay_invisible() {
    let cards = [SkillCard {
        published: false,
        age_years: 34,
        skill: "x".into(),
    }];
    assert!(SkillCard::search(&cards).is_empty());
    assert_eq!(tek_skill(false).unwrap_err(), SkillError::NoGrant);
    assert_eq!(tek_skill(true).unwrap(), "tek-skill-granted");
}

#[test]
fn no_v1_no_minor_rank_no_global_dump() {
    let minor = SkillCard {
        published: true,
        age_years: 16,
        skill: "y".into(),
    };
    assert_eq!(minor.rank_in_marketplace().unwrap_err(), SkillError::ChildRank);
    assert!(!skills_v1_tagged());
    assert!(!global_profile_dump());
}

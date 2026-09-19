use gaia_skills::{global_profile_dump, skills_v1_tagged, tek_skill, SkillCard, SkillError};

#[test]
fn unpublished_invisible_and_minors_not_ranked() {
    let cards = [
        SkillCard {
            published: false,
            age_years: 34,
            skill: "x".into(),
        },
        SkillCard {
            published: true,
            age_years: 16,
            skill: "y".into(),
        },
    ];
    assert_eq!(SkillCard::search(&cards).len(), 1);
    assert_eq!(
        cards[1].rank_in_marketplace().unwrap_err(),
        SkillError::ChildRank
    );
    assert_eq!(tek_skill(false).unwrap_err(), SkillError::NoGrant);
    assert!(!skills_v1_tagged());
    assert!(!global_profile_dump());
}

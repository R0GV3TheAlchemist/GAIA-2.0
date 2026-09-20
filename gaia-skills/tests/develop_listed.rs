//! #497 listed shelf. Existing APIs only.

use gaia_skills::{develop, SkillCard, VaultProfile};

#[test]
fn goal_change_rebuilds_solo_path_without_network() {
    let first = develop("speak");
    let second = develop("listen");
    assert_ne!(first.goal, second.goal);
    assert!(!first.used_network);
    assert!(!second.used_network);
    let mut vault = VaultProfile::offline();
    vault.set_goal("listen");
    assert_eq!(vault.profile.goals[0], "listen");
}

#[test]
fn local_match_hides_unpublished_cards() {
    let cards = [
        SkillCard {
            published: false,
            age_years: 30,
            skill: "listen".into(),
        },
        SkillCard {
            published: true,
            age_years: 30,
            skill: "listen".into(),
        },
    ];
    let found = SkillCard::search(&cards);
    assert_eq!(found.len(), 1);
    assert!(found[0].published);
}

use gaia_skills::{develop, VaultProfile};

#[test]
fn solo_path_has_practice_and_goal_change_rebuilds() {
    let first = develop("speak");
    assert!(first.steps.iter().any(|s| s.kind == "practice"));
    assert!(!first.ukd_gaps.is_empty());
    assert!(!first.used_network);
    let second = develop("listen");
    assert_ne!(first.goal, second.goal);
    let mut vault = VaultProfile::offline();
    vault.set_goal("listen");
    assert_eq!(vault.profile.goals[0], "listen");
}

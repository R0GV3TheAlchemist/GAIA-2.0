//! #495 listed shelf. Existing APIs only.

use gaia_skills::{VaultProfile, SkillError};

#[test]
fn offline_profile_crud_does_not_sync() {
    let mut v = VaultProfile::offline();
    assert!(!v.profile.sync);
    assert!(!v.clusters);
    v.set_goal("novice public speaking");
    assert_eq!(v.profile.goals, ["novice public speaking"]);
    v.delete();
    assert!(v.profile.goals.is_empty());
    assert!(!v.clusters);
}

#[test]
fn cluster_toggle_is_explicit_and_blocks_child_ei() {
    let mut v = VaultProfile::offline();
    assert_eq!(
        v.toggle_clusters(true, 12).unwrap_err(),
        SkillError::ChildEiBlocked
    );
    v.toggle_clusters(true, 18).unwrap();
    assert!(v.clusters);
    assert_eq!(
        v.profile.enable_sync(false).unwrap_err(),
        SkillError::SyncDenied
    );
}

use gaia_skills::{SkillError, SkillProfile};

#[test]
fn profile_stays_local_and_child_cannot_infer_ei() {
    let mut profile = SkillProfile::local();
    assert!(!profile.sync);
    assert_eq!(profile.enable_sync(false).unwrap_err(), SkillError::SyncDenied);
    assert_eq!(profile.enable_inferred_se(12).unwrap_err(), SkillError::ChildEiBlocked);
    profile.enable_inferred_se(34).unwrap();
}

use gaia_skills::novice_public_speaking;

#[test]
fn public_speaking_path_has_open_resource_and_ukd_link() {
    let path = novice_public_speaking();
    assert_eq!(path.goal, "novice public speaking");
    assert!(path.steps.iter().any(|s| s.resource.starts_with("https://")));
    assert!(path.steps.iter().any(|s| s.requires_knowledge.as_deref().unwrap_or("").starts_with("ukd:")));
}

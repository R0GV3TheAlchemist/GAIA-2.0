//! #493 listed shelf. Existing APIs only.

use gaia_skills::{develop, novice_public_speaking};

#[test]
fn public_speaking_lists_practice_resources_and_ukd_prereqs() {
    let path = novice_public_speaking();
    assert!(path.steps.iter().any(|s| s.resource.starts_with("https://")));
    assert!(path.steps.iter().any(|s| s
        .requires_knowledge
        .as_deref()
        .unwrap_or("")
        .starts_with("ukd:")));
    assert!(path.steps.iter().all(|s| s.id.starts_with("skill:")));
}

#[test]
fn develop_is_practice_not_readings_only_and_surfaces_ukd_gaps() {
    let p = develop("novice public speaking");
    assert!(p.steps.iter().any(|s| s.kind == "practice"));
    assert!(!p.ukd_gaps.is_empty());
    assert!(p.ukd_gaps.iter().all(|g| g.starts_with("ukd:")));
    assert!(!p.used_network);
}

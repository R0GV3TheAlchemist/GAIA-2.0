//! #491 listed shelf. Existing APIs only.

use gaia_skills::{active_listening, realm_stubs, skills_v1_tagged, REALMS};

#[test]
fn active_listening_is_a_skill_id_not_ukd() {
    let n = active_listening();
    assert_eq!(n.id, "skill:active-listening");
    assert!(!n.id.starts_with("ukd:"));
    assert_eq!(n.realm, "communication");
    assert_eq!(n.dreyfus, "novice");
    assert!(n.esco.is_none());
    assert!(n.onet.is_none());
    assert_eq!(n.license, "CC0-1.0");
}

#[test]
fn twelve_realms_have_domain_stubs() {
    assert_eq!(REALMS.len(), 12);
    assert!(REALMS.contains(&"digital"));
    assert_eq!(realm_stubs().len(), 12);
    assert!(!skills_v1_tagged());
}

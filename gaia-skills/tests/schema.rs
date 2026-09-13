use gaia_skills::{active_listening, REALMS};

#[test]
fn twelve_realms_and_active_listening_validates() {
    assert_eq!(REALMS.len(), 12);
    let node = active_listening();
    assert_eq!(node.id, "skill:active-listening");
    assert!(node.esco.is_none());
    assert!(!node.license.is_empty());
}

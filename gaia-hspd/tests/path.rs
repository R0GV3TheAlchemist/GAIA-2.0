use gaia_hspd::{flow_path, freediving_note, pharma_path};

#[test]
fn flow_has_no_supplements_and_freediving_is_not_a_protocol() {
    let path = flow_path();
    assert!(!path.supplements);
    assert!(!freediving_note().contains("hold"));
    assert!(!freediving_note().contains("protocol"));
    assert!(pharma_path().is_err());
}

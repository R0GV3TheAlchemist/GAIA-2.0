use gaia_ukd::REALMS;

#[test]
fn twelve_realms_exist_as_data() {
    assert_eq!(REALMS.len(), 12);
    assert!(REALMS.contains(&"traditional-knowledge"));
}

use gaia_hmgd::{principles, prohibited, sealed_rite};

#[test]
fn ethics_and_empty_sealed() {
    assert_eq!(principles().len(), 6);
    assert!(prohibited().iter().any(|p| p.contains("prayer-as-emergency")));
    assert!(sealed_rite().is_err());
}

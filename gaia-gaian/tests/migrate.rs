use gaia_gaian::*;

#[test]
fn migrate_a_to_b_keeps_gaian_id() {
    let id = Identity::new("did:gaian:fixture-1");
    assert_eq!(migrate(&id, "harness-a", "harness-b"), id.gaian_id);
    assert_eq!(model_swap(&id), id.gaian_id);
}

#[test]
fn forgotten_item_is_not_retrievable() {
    let mut mem = PersonalMemory::default();
    mem.put(MemoryTier::Episodic, "secret-note");
    assert!(mem.retrieve("secret-note"));
    mem.forget("secret-note");
    assert!(!mem.retrieve("secret-note"));
    assert!(forget("secret-note").is_none());
}

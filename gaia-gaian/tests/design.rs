use gaia_gaian::{
    default_level, dump_vault, forget, infer_from_photo, migrate, model_swap, pay_at_level,
    Identity,
};

#[test]
fn id_survives_swap_photo_not_inferred_default_l1() {
    let id = Identity::new("gaian-1");
    assert_eq!(model_swap(&id), "gaian-1");
    assert_eq!(migrate(&id, "A", "B"), "gaian-1");
    assert!(infer_from_photo().is_err());
    assert!(forget("secret").is_none());
    assert_eq!(default_level(), 1);
    assert!(pay_at_level(1).is_err());
    assert!(dump_vault().is_err());
}

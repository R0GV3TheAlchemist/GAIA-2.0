use gaia_hmgd::{hmgd_v1_tagged, join_room, mine_denomination, sell_closed_rite, HmgdProfile};

#[test]
fn empty_profile_works_outsiders_not_enrolled_no_sale_no_v1() {
    assert!(HmgdProfile::new().gaian_works_empty());
    assert!(!join_room(false).unwrap().enrolled);
    assert!(sell_closed_rite().is_err());
    assert!(mine_denomination("baptist").is_err());
    assert!(!hmgd_v1_tagged());
}

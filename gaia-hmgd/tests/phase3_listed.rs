//! #537 listed shelf. Existing APIs only.

use gaia_hmgd::{
    hmgd_v1_tagged, join_room, sacred_layer, sell_closed_rite, songlines, HmgdError,
};

#[test]
fn unjoined_users_are_not_enrolled() {
    assert!(!join_room(false).unwrap().enrolled);
    assert!(join_room(true).unwrap().enrolled);
}

#[test]
fn closed_sale_and_songlines_and_v1_stay_gated() {
    assert_eq!(sell_closed_rite().unwrap_err(), HmgdError::SaleForbidden);
    assert_eq!(songlines().unwrap_err(), HmgdError::Sealed);
    assert_eq!(sacred_layer(false).unwrap_err(), HmgdError::NotOptIn);
    assert!(!hmgd_v1_tagged());
}

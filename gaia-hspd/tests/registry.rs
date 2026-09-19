use gaia_hspd::{checkout, hspd_v1_tagged, list_for};

#[test]
fn no_checkout_no_minor_upsell_no_v1() {
    assert!(list_for(12)
        .iter()
        .all(|i| !i.name.contains("openbci") && !i.checkout));
    assert_eq!(
        checkout("implant").unwrap_err(),
        gaia_hspd::HspdError::CheckoutForbidden
    );
    assert!(!hspd_v1_tagged());
}

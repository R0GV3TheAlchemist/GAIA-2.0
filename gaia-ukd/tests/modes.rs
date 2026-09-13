use gaia_ukd::{earth_twin_cites, teach_offline, tek_export, ukd_v1_tagged, GaianMode, UkdError};

#[test]
fn five_modes_offline_teach_and_no_v1() {
    assert_eq!(GaianMode::all().len(), 5);
    assert!(teach_offline(true, "X").unwrap().contains("offline"));
    assert!(earth_twin_cites("ukd:eng:quantum-computing").unwrap().contains("ukd:"));
    assert_eq!(tek_export(false).unwrap_err(), UkdError::NoAgreement);
    assert!(!ukd_v1_tagged());
}

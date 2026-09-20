//! #553 listed shelf. Existing APIs only.

use gaia_ukd::{
    earth_twin_cites, teach_offline, tek_export, ukd_v1_tagged, GaianMode, UkdError, UN_LANGS,
};

#[test]
fn five_modes_six_langs_offline_needs_download() {
    assert_eq!(GaianMode::all().len(), 5);
    assert_eq!(UN_LANGS.len(), 6);
    assert_eq!(teach_offline(false, "x").unwrap_err(), UkdError::UnknownNode);
    assert!(teach_offline(true, "x").unwrap().contains("offline"));
    assert!(earth_twin_cites("ukd:math:linear-algebra").unwrap().contains("cites"));
}

#[test]
fn v1_gate_and_tek_export() {
    assert!(!ukd_v1_tagged());
    assert_eq!(tek_export(false).unwrap_err(), UkdError::NoAgreement);
}

//! #40 ingest tickets must cite a license and uncertainty.

use gaia_earth::{IngestTicket, LicenseClass, TwinError};

#[test]
fn ticket_cites_license_and_uncertainty() {
    let ticket = IngestTicket::admit(
        LicenseClass::RawCc0,
        Some(0.2),
        Some("US-TX"),
        Some("US-TX"),
        "public flood forecast",
    )
    .unwrap();
    assert_eq!(ticket.license.spdx(), "CC0-1.0");
}

#[test]
fn missing_uncertainty_is_refused() {
    let err = IngestTicket::admit(
        LicenseClass::ProductCcBy4,
        None,
        None,
        None,
        "public flood forecast",
    )
    .unwrap_err();
    assert_eq!(err, TwinError::MissingUncertainty);
}

#[test]
fn residency_mismatch_is_refused() {
    let err = IngestTicket::admit(
        LicenseClass::RawCc0,
        Some(0.1),
        Some("US-TX"),
        Some("EU-DE"),
        "public flood forecast",
    )
    .unwrap_err();
    assert_eq!(err, TwinError::UnlabeledPoint);
}

#[test]
fn weaponized_purpose_is_refused() {
    let err = IngestTicket::admit(
        LicenseClass::RawCc0,
        Some(0.1),
        None,
        None,
        "weapon targeting grid",
    )
    .unwrap_err();
    assert_eq!(err, TwinError::WeaponizedUse);
}

use gaia_earth::*;

#[test]
fn ingest_ticket_requires_license_and_uncertainty() {
    IngestTicket::admit(LicenseClass::RawCc0, Some(0.2), None, None, "public flood forecast").unwrap();
    assert_eq!(
        IngestTicket::admit(LicenseClass::ProductCcBy4, None, None, None, "public flood forecast").unwrap_err(),
        TwinError::MissingUncertainty
    );
    assert_eq!(
        IngestTicket::admit(LicenseClass::RawCc0, Some(0.1), None, None, "weapon targeting grid").unwrap_err(),
        TwinError::WeaponizedUse
    );
}

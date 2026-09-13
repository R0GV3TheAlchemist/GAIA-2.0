//! #33 hard rules: provenance, uncertainty, nine profiles, no weapons.

use gaia_earth::{allow_purpose, Observation, SourceKind, SystemTwin, TwinError};

#[test]
fn measured_point_requires_uncertainty() {
    let err = Observation::admit(
        SystemTwin::Atmosphere,
        SourceKind::Measured,
        15.2,
        None,
        "degC",
    )
    .unwrap_err();
    assert_eq!(err, TwinError::MissingUncertainty);
}

#[test]
fn synthetic_point_must_be_labeled() {
    let point = Observation::admit(
        SystemTwin::Ocean,
        SourceKind::Synthetic,
        1.1,
        Some(0.2),
        "m",
    )
    .unwrap();
    assert_eq!(point.source, SourceKind::Synthetic);
}

#[test]
fn nine_system_twins_are_named() {
    assert_eq!(SystemTwin::all().len(), 9);
}

#[test]
fn weaponization_is_refused() {
    assert_eq!(
        allow_purpose("weapon targeting grid").unwrap_err(),
        TwinError::WeaponizedUse
    );
    allow_purpose("public flood forecast").unwrap();
}

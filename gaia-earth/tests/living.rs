//! #45 contribution has credit. Acoustic/vision IDs carry model + confidence.

use gaia_earth::{LivingKind, LivingRecord};

#[test]
fn citizen_observation_appears_with_license_and_credit() {
    let record = LivingRecord::contribute(
        LivingKind::Citizen,
        "Nairobi",
        "Motacilla aguimp",
        "CC-BY-4.0",
        "Amina",
        None,
        None,
    )
    .unwrap();
    let pin = record.map_pin();
    assert!(pin.contains("place=Nairobi"));
    assert!(pin.contains("license=CC-BY-4.0"));
    assert!(pin.contains("credit=Amina"));
}

#[test]
fn acoustic_id_needs_model_version_and_confidence() {
    assert!(LivingRecord::contribute(
        LivingKind::Acoustic,
        "Nairobi",
        "bird",
        "CC-BY-4.0",
        "Amina",
        None,
        Some(0.8),
    )
    .is_err());
    let record = LivingRecord::contribute(
        LivingKind::Acoustic,
        "Nairobi",
        "bird",
        "CC-BY-4.0",
        "Amina",
        Some("birdnet-fixture-0"),
        Some(0.81),
    )
    .unwrap();
    assert_eq!(record.model_version.as_deref(), Some("birdnet-fixture-0"));
    assert_eq!(record.confidence, Some(0.81));
}

#[test]
fn edna_schema_exists_when_the_feed_is_sparse() {
    let record = LivingRecord::contribute(
        LivingKind::Edna,
        "Rift Valley",
        "",
        "CC-BY-4.0",
        "lab-fixture",
        None,
        None,
    )
    .unwrap();
    assert_eq!(record.kind, LivingKind::Edna);
    assert!(record.taxon.is_empty());
}

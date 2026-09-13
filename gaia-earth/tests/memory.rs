//! #36 place-time query returns labeled cubes and versioned model stubs.

use gaia_earth::{PlanetaryMemory, SourceKind, SystemTwin};

#[test]
fn query_returns_signed_cube_and_synthetic_models() {
    let mut memory = PlanetaryMemory::new();
    memory
        .store(
            "Austin",
            1_725_000_000,
            SystemTwin::Atmosphere,
            SourceKind::Measured,
            31.2,
            0.4,
            "degC",
        )
        .unwrap();
    let set = memory.query("Austin", 1_725_000_000).unwrap();
    assert_eq!(set.cubes.len(), 1);
    assert!(set.cubes[0].signed);
    assert_eq!(set.cubes[0].observation.source, SourceKind::Measured);
    assert!(!set.cubes[0].fingerprint.is_empty());
    assert!(set.models.iter().any(|m| m.name == "GraphCast"));
    assert!(set.models.iter().all(|m| m.source == SourceKind::Synthetic));
    assert!(set.models.iter().all(|m| m.uncertainty > 0.0));
    assert!(set.models.iter().all(|m| m.version == "fixture-0"));
}

#[test]
fn unknown_place_time_is_not_invented() {
    let memory = PlanetaryMemory::new();
    assert!(memory.query("Nowhere", 0).is_err());
}

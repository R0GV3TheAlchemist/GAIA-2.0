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
    assert!(set.cubes[0].signed);
    assert!(set.models.iter().any(|m| m.name == "GraphCast"));
    assert!(set.models.iter().all(|m| m.source == SourceKind::Synthetic));
}

#[test]
fn unknown_place_time_is_not_invented() {
    assert!(PlanetaryMemory::new().query("Nowhere", 0).is_err());
}

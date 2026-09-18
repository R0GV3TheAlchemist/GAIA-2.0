use gaia_earth::{
    MemoryTier, Observation, SourceKind, SystemTwin, TierCube, TierStore,
};

fn synthetic_observation(value: f64, unit: &str) -> Observation {
    Observation::admit(
        SystemTwin::Atmosphere,
        SourceKind::Synthetic,
        value,
        Some(1.0),
        unit,
    )
    .unwrap()
}

fn cube(id: &str, tier: MemoryTier, place: &str, value: f64) -> TierCube {
    TierCube {
        id: id.into(),
        tier,
        place: place.into(),
        observation: synthetic_observation(value, "ppm"),
        associations: vec![],
    }
}

#[test]
fn same_api_fetches_paleo_and_live_fixture_records() {
    let mut store = TierStore::new();
    store
        .put(cube(
            "ice-core-co2-800kya",
            MemoryTier::DeepTime,
            "antarctica",
            180.0,
        ))
        .unwrap();
    store
        .put(cube(
            "mauna-loa-co2-now",
            MemoryTier::RealTime,
            "mauna-loa",
            425.0,
        ))
        .unwrap();

    assert_eq!(
        store
            .fetch("antarctica", MemoryTier::DeepTime)
            .unwrap()
            .id,
        "ice-core-co2-800kya"
    );
    assert_eq!(
        store
            .fetch("mauna-loa", MemoryTier::RealTime)
            .unwrap()
            .id,
        "mauna-loa-co2-now"
    );
}

#[test]
fn cross_tier_association_links_paleo_to_live_fixture() {
    let mut store = TierStore::new();
    store
        .put(cube(
            "ice-core-co2-800kya",
            MemoryTier::DeepTime,
            "antarctica",
            180.0,
        ))
        .unwrap();
    store
        .put(cube(
            "mauna-loa-co2-now",
            MemoryTier::RealTime,
            "mauna-loa",
            425.0,
        ))
        .unwrap();

    store
        .associate("ice-core-co2-800kya", "mauna-loa-co2-now")
        .unwrap();

    assert_eq!(
        store
            .fetch("antarctica", MemoryTier::DeepTime)
            .unwrap()
            .associations,
        vec!["mauna-loa-co2-now"]
    );
}

#[test]
fn association_requires_known_distinct_target() {
    let mut store = TierStore::new();
    store
        .put(cube(
            "ice-core-co2-800kya",
            MemoryTier::DeepTime,
            "antarctica",
            180.0,
        ))
        .unwrap();

    assert!(store
        .associate("ice-core-co2-800kya", "missing-cube")
        .is_err());
    assert!(store
        .associate("ice-core-co2-800kya", "ice-core-co2-800kya")
        .is_err());
}

#[test]
fn duplicate_ids_are_rejected() {
    let mut store = TierStore::new();
    store
        .put(cube(
            "fixture-id",
            MemoryTier::DeepTime,
            "antarctica",
            180.0,
        ))
        .unwrap();

    assert!(store
        .put(cube(
            "fixture-id",
            MemoryTier::RealTime,
            "mauna-loa",
            425.0,
        ))
        .is_err());
}

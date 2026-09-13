use gaia_earth::{MemoryTier, Observation, SourceKind, SystemTwin, TierCube, TierStore};

fn cube(id: &str, place: &str, tier: MemoryTier) -> TierCube {
    TierCube {
        id: id.into(),
        tier,
        place: place.into(),
        observation: Observation::admit(
            SystemTwin::Atmosphere,
            SourceKind::Measured,
            1.0,
            Some(0.1),
            "ppm",
        )
        .unwrap(),
        associations: vec![],
    }
}

#[test]
fn same_api_fetches_paleo_and_live() {
    let mut store = TierStore::new();
    store.put(cube("ice", "Antarctica", MemoryTier::DeepTime));
    store.put(cube("mlo", "Mauna Loa", MemoryTier::RealTime));
    assert!(store.fetch("Antarctica", MemoryTier::DeepTime).is_ok());
    assert!(store.fetch("Mauna Loa", MemoryTier::RealTime).is_ok());
}

#[test]
fn association_links_ice_core_to_mauna_loa() {
    let mut store = TierStore::new();
    store.put(cube("ice", "Antarctica", MemoryTier::DeepTime));
    store.put(cube("mlo", "Mauna Loa", MemoryTier::RealTime));
    store.associate("ice", "mlo").unwrap();
    assert!(store
        .fetch("Antarctica", MemoryTier::DeepTime)
        .unwrap()
        .associations
        .contains(&"mlo".into()));
}

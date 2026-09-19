use gaia_earth::*;

#[test]
fn raw_is_immutable_and_curated_exists() {
    let mut lake = Lake::new();
    let obs = Observation::admit(SystemTwin::Land, SourceKind::Synthetic, 0.4, Some(0.1), "ndvi").unwrap();
    lake.write_raw("stac-demo", obs.clone()).unwrap();
    assert_eq!(
        lake.write_raw("stac-demo", obs).unwrap_err(),
        TwinError::ImmutableRaw
    );
    lake.promote("stac-demo").unwrap();
    assert_eq!(lake.curated().len(), 1);
    assert!(Lake::stac_collection("stac-demo").contains("stac_version"));
    assert!(Lake::openeo_graph("stac-demo").contains("load_collection"));
}

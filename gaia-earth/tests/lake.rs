//! #41 Raw is immutable. Curated is derived. STAC/openEO are stubs.

use gaia_earth::{Lake, LakeZone, Observation, SourceKind, SystemTwin, TwinError};

fn point() -> Observation {
    Observation::admit(
        SystemTwin::Land,
        SourceKind::Synthetic,
        0.4,
        Some(0.1),
        "ndvi",
    )
    .unwrap()
}

#[test]
fn write_lands_in_raw_and_promotes_to_curated() {
    let mut lake = Lake::new();
    let raw = lake.write_raw("sentinel-2-l1c-fixture", point()).unwrap();
    assert_eq!(raw.zone, LakeZone::Raw);
    assert!(raw.doi.contains("gaia:10.placeholder"));
    let curated = lake.promote("sentinel-2-l1c-fixture").unwrap();
    assert_eq!(curated.zone, LakeZone::Curated);
    assert_eq!(lake.raw().len(), 1);
    assert_eq!(lake.curated().len(), 1);
}

#[test]
fn raw_cannot_be_overwritten() {
    let mut lake = Lake::new();
    lake.write_raw("same", point()).unwrap();
    let err = lake.write_raw("same", point()).unwrap_err();
    assert_eq!(err, TwinError::ImmutableRaw);
}

#[test]
fn stac_and_openeo_stubs_name_the_collection() {
    let stac = Lake::stac_collection("sentinel-2-l1c-fixture");
    assert!(stac.contains("stac_version=1.0.0"));
    assert!(stac.contains("sentinel-2-l1c-fixture"));
    assert!(Lake::openeo_graph("sentinel-2-l1c-fixture").contains("load_collection"));
}

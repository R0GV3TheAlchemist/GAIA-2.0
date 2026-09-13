use gaia_earth::{FeedKind, NervousFabric, QcTier, SourceKind, SystemTwin, TwinError};

#[test]
fn heterogeneous_feeds_land_with_full_metadata() {
    let mut fabric = NervousFabric::new();
    fabric
        .ingest(
            "cell-austin",
            SystemTwin::Atmosphere,
            SourceKind::Measured,
            31.2,
            0.4,
            "degC",
            FeedKind::InSitu,
            "fixture ground station",
            1_725_000_000,
            QcTier::L2Calibrated,
        )
        .unwrap();
    let samples = fabric.read_cell("cell-austin").unwrap();
    assert_eq!(samples[0].observation.uncertainty, 0.4);
    assert_eq!(samples[0].quality, QcTier::L2Calibrated);
}

#[test]
fn sparse_region_is_flagged_not_invented() {
    let err = NervousFabric::new().read_cell("cell-empty").unwrap_err();
    assert_eq!(err, TwinError::SparseRegion);
}

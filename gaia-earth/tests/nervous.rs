//! #35 curated samples carry value, uncertainty, provenance, time, QC.
//! Sparse cells are flagged.

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
    fabric
        .ingest(
            "cell-austin",
            SystemTwin::Biosphere,
            SourceKind::Measured,
            1.0,
            0.2,
            "count",
            FeedKind::Citizen,
            "fixture citizen report",
            1_725_000_100,
            QcTier::L1Screened,
        )
        .unwrap();
    let samples = fabric.read_cell("cell-austin").unwrap();
    assert_eq!(samples.len(), 2);
    assert_eq!(samples[0].observation.uncertainty, 0.4);
    assert_eq!(samples[0].timestamp_unix, 1_725_000_000);
    assert_eq!(samples[0].quality, QcTier::L2Calibrated);
    assert!(!samples[0].provenance.is_empty());
    assert_eq!(samples[1].feed, FeedKind::Citizen);
}

#[test]
fn sparse_region_is_flagged_not_invented() {
    let fabric = NervousFabric::new();
    let err = fabric.read_cell("cell-empty").unwrap_err();
    assert_eq!(err, TwinError::SparseRegion);
    assert!(format!("{err}").contains("not invented"));
}

#[test]
fn five_qc_tiers_exist() {
    let tiers = [
        QcTier::L0Raw,
        QcTier::L1Screened,
        QcTier::L2Calibrated,
        QcTier::L3Assimilated,
        QcTier::L4Serving,
    ];
    assert_eq!(tiers.len(), 5);
}

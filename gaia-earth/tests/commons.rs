//! #34 four fixture collections with provenance and quality. No live APIs.

use gaia_earth::{Commons, Domain, QualityTier, SourceKind};

#[test]
fn catalog_has_satellite_weather_biodiversity_and_seismic() {
    let commons = Commons::seed();
    let domains: Vec<_> = commons.collections().iter().map(|c| c.domain).collect();
    assert!(domains.contains(&Domain::Satellite));
    assert!(domains.contains(&Domain::Weather));
    assert!(domains.contains(&Domain::Biodiversity));
    assert!(domains.contains(&Domain::SeismicOcean));
}

#[test]
fn query_returns_synthetic_observation_with_uncertainty() {
    let commons = Commons::seed();
    let point = commons
        .query("ground-weather-fixture", 21.4, 0.5, "degC")
        .unwrap();
    assert_eq!(point.source, SourceKind::Synthetic);
    assert_eq!(point.uncertainty, 0.5);
}

#[test]
fn every_collection_has_license_provenance_and_tier() {
    let commons = Commons::seed();
    for collection in commons.collections() {
        assert!(!collection.license.is_empty());
        assert!(!collection.provenance.is_empty());
        assert!(matches!(
            collection.quality,
            QualityTier::Raw | QualityTier::Curated | QualityTier::Enriched
        ));
    }
}

#[test]
fn stac_item_is_shaped_and_not_a_live_catalog() {
    let item = Commons::seed()
        .stac_item("sentinel-2-l1c-fixture")
        .unwrap();
    assert!(item.contains("stac_version"));
    assert!(item.contains("synthetic fixture"));
}

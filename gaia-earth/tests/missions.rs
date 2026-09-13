//! #44 mission list is machine-readable. Gaps are named. Tick is local.

use gaia_earth::{CoverageGap, MissionCatalog, MissionDomain};

#[test]
fn catalog_lists_space_ocean_and_seismic_missions() {
    let catalog = MissionCatalog::seed();
    let names: Vec<_> = catalog.missions().iter().map(|m| m.name.as_str()).collect();
    assert!(names.contains(&"Sentinel-2"));
    assert!(names.contains(&"ICESat-2"));
    assert!(names.contains(&"Argo"));
    assert!(names.contains(&"NDBC"));
    assert!(names.contains(&"USGS"));
    assert!(names.contains(&"GeoNet"));
    assert!(catalog.missions().iter().all(|m| !m.license.is_empty()));
    assert!(catalog.missions().iter().all(|m| !m.cadence.is_empty()));
}

#[test]
fn coverage_gaps_are_explicit() {
    let gaps = MissionCatalog::gaps();
    assert!(gaps.contains(&CoverageGap::DeepOcean));
    assert!(gaps.contains(&CoverageGap::Poles));
    assert!(gaps.contains(&CoverageGap::LowIncomeRegions));
}

#[test]
fn one_product_per_domain_receives_a_schedule_tick() {
    let mut catalog = MissionCatalog::seed();
    catalog.tick(1_725_000_000);
    for domain in [
        MissionDomain::Space,
        MissionDomain::OceanInSitu,
        MissionDomain::Seismic,
    ] {
        assert!(catalog
            .missions()
            .iter()
            .any(|m| m.domain == domain && m.last_tick == 1_725_000_000));
    }
}

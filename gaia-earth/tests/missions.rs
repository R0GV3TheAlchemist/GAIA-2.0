use gaia_earth::*;

#[test]
fn catalog_lists_space_ocean_seismic_and_gaps() {
    let mut cat = MissionCatalog::seed();
    assert!(cat.missions().len() >= 15);
    assert!(cat.missions().iter().any(|m| m.domain == MissionDomain::Space && m.name == "Sentinel-2"));
    assert!(cat.missions().iter().any(|m| m.domain == MissionDomain::OceanInSitu && m.name == "Argo"));
    assert!(cat.missions().iter().any(|m| m.domain == MissionDomain::Seismic && m.name == "USGS"));
    let gaps = MissionCatalog::gaps();
    assert!(gaps.contains(&CoverageGap::DeepOcean));
    assert!(gaps.contains(&CoverageGap::Poles));
    assert!(gaps.contains(&CoverageGap::LowIncomeRegions));
    cat.tick(42);
    assert!(cat.missions().iter().any(|m| m.domain == MissionDomain::Space && m.last_tick == 42));
    assert!(cat.missions().iter().any(|m| m.domain == MissionDomain::OceanInSitu && m.last_tick == 42));
    assert!(cat.missions().iter().any(|m| m.domain == MissionDomain::Seismic && m.last_tick == 42));
}

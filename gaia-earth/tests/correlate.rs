use gaia_earth::{BoundaryIndicator, CaseStudy};

#[test]
fn ice_jet_drought_is_reproducible_and_not_policy() {
    let study = CaseStudy::ice_jet_drought();
    assert_eq!(study.chain.len(), 3);
    assert!(!study.is_policy());
}

#[test]
fn boundary_indicator_has_owner_unit_and_cadence() {
    let indicator = BoundaryIndicator::climate_co2();
    assert!(!indicator.owner.is_empty());
    assert_eq!(indicator.unit, "ppm");
    assert!(!indicator.cadence.is_empty());
}

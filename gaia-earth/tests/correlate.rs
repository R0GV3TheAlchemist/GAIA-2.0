use gaia_earth::*;

#[test]
fn case_study_is_not_policy() {
    let c = CaseStudy::ice_jet_drought();
    assert_eq!(c.chain.len(), 3);
    assert!(!c.is_policy());
}

#[test]
fn nine_boundary_indicators_have_owner_unit_cadence() {
    let rows = BoundaryIndicator::nine();
    assert_eq!(rows.len(), 9);
    assert!(rows
        .iter()
        .all(|r| !r.owner.is_empty() && !r.unit.is_empty() && !r.cadence.is_empty()));
}

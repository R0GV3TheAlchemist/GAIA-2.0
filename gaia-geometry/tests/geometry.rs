//! Geometry modules: formal shapes + First-Law refusals.

use gaia_geometry::{
    geometry_v1_tagged, lookup, operational, refuse_category_collapse, refuse_command_circle,
    refuse_tek_extract, Alignment, Cardinal, Circle, Compass, Gate, GeoError, Point, SymbolKind,
    Vesica,
};

#[test]
fn vesica_is_an_intersection_not_a_portal() {
    let left = Circle::new(0.0, 0.0, 1.0).unwrap();
    let right = Circle::new(1.0, 0.0, 1.0).unwrap();
    let vesica = Vesica::try_new(left, right).unwrap();
    assert!(vesica.overlap_width() > 0.0);
    assert!(left.contains(Point::new(0.5, 0.0)));
    assert!(!left.is_command());
}

#[test]
fn separated_circles_are_not_a_vesica() {
    let a = Circle::new(0.0, 0.0, 1.0).unwrap();
    let b = Circle::new(5.0, 0.0, 1.0).unwrap();
    assert!(Vesica::try_new(a, b).is_err());
}

#[test]
fn registered_symbols_have_first_law_fields() {
    let diamond = lookup("gaia.diamond.alignment").unwrap();
    assert_eq!(diamond.kind, SymbolKind::FormalGeometry);
    assert!(!diamond.steward.is_empty());
    assert!(diamond.non_claim.contains("Not"));
    assert!(operational("gaia.compass.navigate").is_ok());
    assert_eq!(operational("gaia.flower.of.life"), Err(GeoError::IncompleteContract));
}

#[test]
fn diamond_refuses_an_aggregate_score() {
    let a = Alignment {
        truth: Gate::Pass,
        care: Gate::Pass,
        growth: Gate::Narrow,
        balance: Gate::Pass,
    };
    assert_eq!(a.wisdom(), Gate::Narrow);
    assert_eq!(a.score(), Err(GeoError::AggregateScoreForbidden));
}

#[test]
fn missing_evidence_or_steward_fails_closed() {
    assert_eq!(
        Alignment::refuse_if_no_evidence(false),
        Err(GeoError::MissingEvidence)
    );
    assert_eq!(
        Alignment::refuse_if_no_steward(false),
        Err(GeoError::MissingSteward)
    );
}

#[test]
fn compass_center_is_wisdom_not_sentience() {
    let c = Compass {
        north: Gate::Pass,
        east: Gate::Pass,
        south: Gate::Defer,
        west: Gate::Pass,
    };
    assert_eq!(c.center(), Gate::Defer);
    assert!(!c.is_sentient());
    assert!(Cardinal::NorthTruth.question().contains("uncertain"));
}

#[test]
fn refusals_hold() {
    assert_eq!(refuse_tek_extract(), Err(GeoError::TekSealed));
    assert_eq!(refuse_command_circle(), Err(GeoError::CommandClaim));
    assert_eq!(refuse_category_collapse(), Err(GeoError::CategoryCollapse));
    assert!(!geometry_v1_tagged());
}

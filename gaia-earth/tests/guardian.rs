//! #39 nine boundaries, threshold drill, weaponization refused. Not Twin v1.0.

use gaia_earth::{Boundary, BoundaryStatus, CorrectionStep, Guardian, TwinError};

#[test]
fn nine_boundaries_have_method_and_uncertainty() {
    let guardian = Guardian::seed();
    assert_eq!(guardian.states().len(), 9);
    for state in guardian.states() {
        assert!(!state.method.is_empty());
        assert!(state.uncertainty > 0.0);
        assert_eq!(state.status, BoundaryStatus::Safe);
    }
}

#[test]
fn approaching_threshold_runs_course_correction() {
    let mut guardian = Guardian::seed();
    guardian
        .set(Boundary::ClimateChange, 0.85, 0.05, "fixture-threshold")
        .unwrap();
    let correction = guardian
        .correct(Boundary::ClimateChange, "public flood forecast")
        .unwrap();
    assert_eq!(correction.steps[0], CorrectionStep::Alert);
    assert_eq!(correction.steps[4], CorrectionStep::Track);
    assert!(correction.recommendation.contains("fixture"));
}

#[test]
fn weaponized_purpose_is_refused() {
    let mut guardian = Guardian::seed();
    guardian
        .set(Boundary::ClimateChange, 0.9, 0.1, "fixture-threshold")
        .unwrap();
    let err = guardian
        .correct(Boundary::ClimateChange, "weapon targeting grid")
        .unwrap_err();
    assert_eq!(err, TwinError::WeaponizedUse);
}

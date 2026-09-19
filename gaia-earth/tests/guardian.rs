use gaia_earth::*;

#[test]
fn approaching_boundary_opens_correction_loop() {
    let mut g = Guardian::seed();
    assert_eq!(g.states().len(), 9);
    g.set(Boundary::ClimateChange, 0.9, 0.1, "fixture-ratio").unwrap();
    let c = g.correct(Boundary::ClimateChange, "public flood forecast").unwrap();
    assert_eq!(c.steps.len(), 5);
    assert!(g.correct(Boundary::BiosphereIntegrity, "public flood forecast").is_err());
}

use gaia_aispd::{license_exam, weather_node};

#[test]
fn weather_cites_earth_twin_and_exams_are_reference() {
    let w = weather_node();
    assert!(w.cites.iter().any(|c| c == "#48"));
    assert_eq!(license_exam("USMLE").score_kind, "reference_published");
}

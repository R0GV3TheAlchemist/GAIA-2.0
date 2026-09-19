//! #475 listed shelf. Existing APIs only.

use gaia_aispd::{aispd_v1_tagged, license_exam, weather_node, wet_lab, Watch};

#[test]
fn weather_cites_48_and_104_and_is_not_measured_yet() {
    let w = weather_node();
    assert_eq!(w.id, "aispd:weather:graphcast-class");
    assert!(w.cites.iter().any(|c| c == "#48"));
    assert!(w.cites.iter().any(|c| c == "#104"));
    assert_eq!(w.score_kind, "gaia_measured_pending");
}

#[test]
fn exams_are_reference_published_only() {
    assert_eq!(license_exam("USMLE").score_kind, "reference_published");
    assert_eq!(license_exam("bar").score_kind, "reference_published");
    assert_eq!(wet_lab(), Watch::Denied);
    assert!(!aispd_v1_tagged());
}

use gaia_aikd::{aikd_v1_tagged, measured_vs_published, overconfident_fixture_dropped, release_ready};

#[test]
fn no_v1_and_published_is_not_measured() {
    assert!(!aikd_v1_tagged());
    assert!(release_ready());
    assert!(measured_vs_published().iter().any(|r| r.contains("reference-only")));
    assert!(overconfident_fixture_dropped(true));
}

use gaia_aispd::{aispd_v1_tagged, not_asi_line, pause_drill};

#[test]
fn not_asi_and_pause_drill_and_no_v1() {
    assert!(not_asi_line().contains("not ASI"));
    assert_eq!(pause_drill(), ["disclose", "pause", "review"]);
    assert!(!aispd_v1_tagged());
}

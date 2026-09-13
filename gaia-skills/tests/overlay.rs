use gaia_skills::{realm_stubs, wef_resolves, wef_top10};

#[test]
fn twelve_realms_and_wef_tags_resolve() {
    assert_eq!(realm_stubs().len(), 12);
    assert_eq!(wef_top10().len(), 10);
    for tag in wef_top10() {
        assert!(wef_resolves(tag).starts_with("skill:"));
    }
}

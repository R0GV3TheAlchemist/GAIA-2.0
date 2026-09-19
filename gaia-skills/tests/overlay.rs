use gaia_skills::{bessi_domains, digcomp_areas, realm_stubs, research_realm_bind, research_realms, wef_2025_essay, wef_resolves, wef_top10, REALMS};

#[test]
fn twelve_realms_and_wef_tags_resolve() {
    assert_eq!(realm_stubs().len(), 12);
    assert_eq!(wef_top10().len(), 10);
    for tag in wef_top10() {
        assert!(wef_resolves(tag).starts_with("skill:"));
    }
}

#[test]
fn research_realms_bind_onto_crate_realms() {
    assert_eq!(research_realms().len(), 12);
    for name in research_realms() {
        let bound = research_realm_bind(name);
        assert!(!bound.is_empty(), "{name}");
        for realm in bound {
            assert!(REALMS.contains(realm), "{name}->{realm}");
        }
    }
    assert!(research_realm_bind("not-a-realm").is_empty());
    assert_eq!(wef_2025_essay().len(), 10);
    assert_eq!(digcomp_areas().len(), 5);
    assert_eq!(bessi_domains().len(), 5);
}

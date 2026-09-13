use gaia_ukd::Ingested;

#[test]
fn physics_has_qid_sitelink_and_license() {
    let phys = Ingested::physics_subject().admit().unwrap();
    assert!(phys.wikidata.starts_with('Q'));
    assert!(phys.wikipedia.contains("wikipedia"));
    assert!(!phys.license.is_empty());
    let okg = Ingested::okg_prerequisite().admit().unwrap();
    assert_eq!(okg.relation.as_deref(), Some("PREREQUISITE_OF"));
}

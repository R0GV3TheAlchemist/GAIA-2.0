//! #557 listed shelf. Existing APIs only.

use gaia_ukd::{ukd_v1_tagged, Ingested, UkdError};

#[test]
fn physics_fixture_has_qid_sitelink_license() {
    let p = Ingested::physics_subject();
    assert_eq!(p.subject, "physics");
    assert_eq!(p.wikidata, "Q413");
    assert!(p.wikipedia.contains("wikipedia"));
    assert!(!p.license.is_empty());
    p.admit().unwrap();
}

#[test]
fn okg_prereq_and_empty_license() {
    let o = Ingested::okg_prerequisite();
    assert_eq!(o.relation.as_deref(), Some("PREREQUISITE_OF"));
    let bad = Ingested {
        license: String::new(),
        ..Ingested::physics_subject()
    };
    assert_eq!(bad.admit().unwrap_err(), UkdError::Uncited);
    assert!(!ukd_v1_tagged());
}

use gaia_sa::*;

#[test]
fn consultation_can_return_no() {
    assert_eq!(consult(false).unwrap_err(), SaError::TekVeto);
    assert_eq!(ingest_site(None).unwrap_err(), SaError::NoConsultation);
    assert_eq!(neuro_claim(false).unwrap(), "ungraded");
}

#[test]
fn open_stack_no_rhino_no_fake_lbc() {
    assert!(tools().contains(&"freecad"));
    assert!(!rhino_required());
    assert!(!lbc_certified());
    assert!(!sa_v1_tagged());
    assert_eq!(scrape_country().unwrap_err(), SaError::SealedPlace);
    assert!(!sacred(false));
}

#[test]
fn packet_is_paper_not_a_site() {
    let p = Packet::paper();
    assert_eq!(p.stages, 7);
    assert_eq!(biophilia(), 14);
    assert_eq!(publish_site(false).unwrap_err(), SaError::NoConsultation);
}

#[test]
fn kit_has_no_live_culture_default() {
    assert!(palette().contains(&"timber"));
    assert_eq!(live_culture().unwrap_err(), SaError::LiveCulture);
    assert_eq!(carbon_claim(None).unwrap_err(), SaError::SloganCarbon);
}

#[test]
fn generate_cannot_skip_access() {
    assert_eq!(generate(None).unwrap_err(), SaError::AccessSkipped);
    assert_eq!(passport(), "json-no-proprietary-ledger");
    assert!(reuse_first());
}

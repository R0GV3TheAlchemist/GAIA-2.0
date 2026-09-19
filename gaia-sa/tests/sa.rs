use gaia_sa::{
    biophilia, carbon_claim, consult, ingest_site, lbc_certified, live_culture, neuro_claim,
    palette, publish_site, rhino_required, sa_v1_tagged, sacred, tools, Packet,
};

#[test]
fn tek_veto_no_rhino_no_lbc_no_live_culture() {
    assert_eq!(consult(false).unwrap_err(), gaia_sa::SaError::TekVeto);
    ingest_site(None).unwrap_err();
    assert_eq!(neuro_claim(false).unwrap(), "ungraded");
    assert!(!rhino_required());
    assert!(tools().contains(&"freecad"));
    assert!(!sacred(false));
    assert_eq!(Packet::paper().stages, 7);
    assert_eq!(biophilia(), 14);
    publish_site(false).unwrap_err();
    assert!(palette().contains(&"timber"));
    assert!(live_culture().is_err());
    assert!(carbon_claim(None).is_err());
    assert!(!lbc_certified());
    assert!(!sa_v1_tagged());
}

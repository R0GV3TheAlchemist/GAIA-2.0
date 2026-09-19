use gaia_si::{
    admit, admit_occupancy, advise, aggregate, face_field, hvac_write, life_safety, nature_first,
    pathogen_protocol, review_field, si_v1_tagged, Prefs, Twin,
};

#[test]
fn purpose_required_no_faces_no_hvac_no_pathogen() {
    assert!(admit("").is_err());
    admit("hvac").unwrap();
    assert!(face_field().is_err());
    assert!(admit_occupancy("biometric").is_err());
    assert!(hvac_write().is_err());
    assert!(pathogen_protocol().is_err());
    assert!(review_field(false).is_err());
    assert!(nature_first().contains(&"wetlands"));
    assert!(life_safety(false).is_err());
    assert!(advise("flip dam", false).is_err());
    assert!(aggregate(&Twin::building()).is_none());
    assert!(Twin::building().interval);
    let _ = Prefs::default().wipe();
    assert!(!si_v1_tagged());
}

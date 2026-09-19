//! #451 / #453 scale-layer bind. Calls APIs already on main.
use gaia_gaian::honesty::{gaian_v1_tagged, live_ollama, live_whisper, medical_product};
use gaia_gaian::{
    ambient_listen, child_level, default_level, infer_from_photo, Consent, ConsentScope, SampleKind,
};

#[test]
fn scale_gaian_is_record_not_person() {
    assert!(!gaian_v1_tagged());
    assert!(!medical_product());
    assert!(!live_ollama());
    assert!(!live_whisper());
    assert_eq!(default_level(), 1);
    assert!(infer_from_photo().is_err());
    assert!(child_level(2).is_err());
    assert!(ambient_listen(true).is_err());
    assert_eq!(ConsentScope::all().len(), 7);
    let under = Consent {
        subject_is_self: true,
        age_years: 12,
        self_consent: true,
        parental_consent: true,
        health_opt_in: true,
    };
    assert!(gaia_gaian::create_self(&under, SampleKind::Photo).is_err());
}

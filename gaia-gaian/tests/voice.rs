use gaia_gaian::{capture_app_allows, Appearance, Consent, GaianError, VoiceProfile};

fn adult() -> Consent {
    Consent {
        subject_is_self: true,
        age_years: 34,
        self_consent: true,
        parental_consent: false,
        health_opt_in: false,
    }
}

#[test]
fn local_tts_and_appearance_reset_without_reupload() {
    let voice = VoiceProfile::capture(&adult()).unwrap();
    assert!(voice.speak("hello").unwrap().contains("local-tts"));
    assert_eq!(Appearance::reset().mode, "default-without-raw-reupload");
    assert_eq!(
        capture_app_allows(false).unwrap_err(),
        GaianError::NoConsent
    );
}

use gaia_gaian::{Consent, DigitalVault, GaianError, KnowKind};

fn adult() -> Consent {
    Consent { subject_is_self: true, age_years: 34, self_consent: true, parental_consent: false, health_opt_in: false }
}
fn child() -> Consent {
    Consent { subject_is_self: true, age_years: 12, self_consent: true, parental_consent: true, health_opt_in: false }
}

#[test]
fn offline_talk_show_and_wipe() {
    assert!(DigitalVault::talk_offline(&adult(), "hi").unwrap().starts_with("offline:"));
    let mut vault = DigitalVault::default();
    vault.store(KnowKind::Preferences, "tea");
    vault.wipe();
    assert!(vault.show_what_you_know().is_empty());
    assert_eq!(vault.enable_learning(&child()).unwrap_err(), GaianError::Under16);
}

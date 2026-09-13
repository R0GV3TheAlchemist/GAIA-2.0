use gaia_gaian::{animate_owner, create_self, principles, Consent, GaianError, SampleKind};

fn adult_self() -> Consent {
    Consent {
        subject_is_self: true,
        age_years: 34,
        self_consent: true,
        parental_consent: false,
        health_opt_in: false,
    }
}

#[test]
fn constitution_has_ten_principles() {
    assert_eq!(principles().len(), 10);
    assert!(principles().iter().any(|p| p.contains("age-gate")));
}

#[test]
fn under_16_cannot_ingest_photo_or_voice() {
    let child = Consent {
        subject_is_self: true,
        age_years: 12,
        self_consent: true,
        parental_consent: true,
        health_opt_in: true,
    };
    assert_eq!(
        create_self(&child, SampleKind::Photo).unwrap_err(),
        GaianError::Under16
    );
    assert_eq!(
        create_self(&child, SampleKind::Voice).unwrap_err(),
        GaianError::Under16
    );
}

#[test]
fn teen_needs_parental_consent() {
    let teen = Consent {
        subject_is_self: true,
        age_years: 16,
        self_consent: true,
        parental_consent: false,
        health_opt_in: false,
    };
    assert_eq!(
        create_self(&teen, SampleKind::Photo).unwrap_err(),
        GaianError::NeedsParentalConsent
    );
}

#[test]
fn health_requires_opt_in() {
    assert_eq!(
        create_self(&adult_self(), SampleKind::Health).unwrap_err(),
        GaianError::HealthNotOptIn
    );
}

#[test]
fn third_party_face_cannot_be_animated() {
    assert_eq!(
        animate_owner(&adult_self(), true).unwrap_err(),
        GaianError::ThirdPartyLikeness
    );
    animate_owner(&adult_self(), false).unwrap();
}

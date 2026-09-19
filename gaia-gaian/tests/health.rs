use gaia_gaian::{age_progress_own, future_self, Consent, GaianError, HealthMetric, HealthTwin};

fn adult_health() -> Consent {
    Consent {
        subject_is_self: true,
        age_years: 34,
        self_consent: true,
        parental_consent: false,
        health_opt_in: true,
    }
}

#[test]
fn health_defaults_off_and_blocks_insurer_export() {
    let mut twin = HealthTwin::default_off();
    assert!(!twin.enabled);
    assert_eq!(
        twin.admit(&adult_health(), HealthMetric::Steps, 1.0, true)
            .unwrap_err(),
        GaianError::HealthDefaultOff
    );
    twin.enable(&adult_health()).unwrap();
    twin.admit(&adult_health(), HealthMetric::Steps, 1.0, true)
        .unwrap();
    assert_eq!(
        twin.export_insurer().unwrap_err(),
        GaianError::ExportForbidden
    );
    assert_eq!(
        twin.export_employer().unwrap_err(),
        GaianError::ExportForbidden
    );
}

#[test]
fn future_self_is_not_medical_advice() {
    assert_eq!(
        future_self("diagnose this rash").unwrap_err(),
        GaianError::NotMedicalAdvice
    );
    assert!(future_self("what if I walk more")
        .unwrap()
        .contains("not medical"));
}

#[test]
fn genetic_upload_is_isolated() {
    let mut twin = HealthTwin::default_off();
    twin.isolate_genetic_upload();
    assert!(twin.genetic_is_isolated());
}

#[test]
fn age_progress_is_self_only() {
    age_progress_own(&adult_health()).unwrap();
}

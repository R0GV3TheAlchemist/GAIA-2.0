use gaia_gaian::{Consent, GaianError, HealthModule};

fn adult_health() -> Consent {
    Consent {
        subject_is_self: true,
        age_years: 34,
        self_consent: true,
        parental_consent: false,
        health_opt_in: true,
    }
}

fn child() -> Consent {
    Consent {
        subject_is_self: true,
        age_years: 12,
        self_consent: true,
        parental_consent: true,
        health_opt_in: true,
    }
}

#[test]
fn health_off_on_first_launch_and_child_cannot_learn() {
    let mut module = HealthModule::first_launch();
    assert!(!module.enabled);
    assert!(!module.is_diagnostic_device());
    assert_eq!(
        module.enable_learning(&child()).unwrap_err(),
        GaianError::Under16
    );
    module.enable_learning(&adult_health()).unwrap();
    module.export("personal review").unwrap();
    assert!(module.log().iter().any(|e| e.contains("purpose=")));
    assert!(module.export("insurer underwriting").is_err());
}

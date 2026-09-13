use gaia_gaian::{equity_eval_labels, server_face_store, CaptureSession, Consent, GaianError};

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
fn owner_photo_stays_in_vault_and_other_person_is_refused() {
    let session = CaptureSession::capture(&adult(), true, false, false).unwrap();
    assert!(session.raw_in_vault);
    assert_eq!(session.vrm_name, "local-stub-vrm");
    assert_eq!(
        CaptureSession::capture(&adult(), true, true, false).unwrap_err(),
        GaianError::NotSelf
    );
    assert!(server_face_store().is_empty());
    assert!(equity_eval_labels().len() >= 4);
}

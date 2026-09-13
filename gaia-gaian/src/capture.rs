//! #67 self-only capture stub. Not rembg, MediaPipe, or SMPL-X.

use crate::{Consent, GaianError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureSession {
    pub raw_in_vault: bool,
    pub export_consented: bool,
    pub vrm_name: String,
}

impl CaptureSession {
    pub fn capture(
        consent: &Consent,
        owner_photo: bool,
        looks_like_other_person: bool,
        export_consented: bool,
    ) -> Result<Self, GaianError> {
        if !consent.self_consent {
            return Err(GaianError::NoConsent);
        }
        if consent.age_years < 16 {
            return Err(GaianError::Under16);
        }
        if !owner_photo || !consent.subject_is_self || looks_like_other_person {
            return Err(GaianError::NotSelf);
        }
        Ok(Self {
            raw_in_vault: !export_consented,
            export_consented,
            vrm_name: "local-stub-vrm".into(),
        })
    }
}

pub fn server_face_store() -> [&'static str; 0] {
    []
}

pub fn equity_eval_labels() -> [&'static str; 4] {
    [
        "multiple-skin-tones-required",
        "multiple-body-types-required",
        "no-default-eurocentric-mesh",
        "limits-documented",
    ]
}

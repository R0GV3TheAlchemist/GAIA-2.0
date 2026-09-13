//! #68 own-voice and appearance stub. Not Kokoro, XTTS, or Flutter.

use crate::{Consent, GaianError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceProfile {
    pub local_only: bool,
    pub ready: bool,
}

impl VoiceProfile {
    pub fn capture(consent: &Consent) -> Result<Self, GaianError> {
        if !consent.self_consent || !consent.subject_is_self {
            return Err(GaianError::NoConsent);
        }
        if consent.age_years < 16 {
            return Err(GaianError::Under16);
        }
        Ok(Self {
            local_only: true,
            ready: true,
        })
    }

    pub fn speak(&self, sentence: &str) -> Result<String, GaianError> {
        if !self.local_only || !self.ready {
            return Err(GaianError::NoConsent);
        }
        Ok(format!("local-tts-fixture: {sentence}"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Appearance {
    pub mode: String,
}

impl Appearance {
    pub fn reset() -> Self {
        Self {
            mode: "default-without-raw-reupload".into(),
        }
    }
}

pub fn capture_app_allows(consent_present: bool) -> Result<(), GaianError> {
    if !consent_present {
        return Err(GaianError::NoConsent);
    }
    Ok(())
}

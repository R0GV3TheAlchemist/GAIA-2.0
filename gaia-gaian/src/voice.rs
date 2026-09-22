//! #68 own-voice and appearance.
//!
//! `VoiceProfile::transcribe` now routes to the real whisper-rs ASR path
//! (when compiled with `--features whisper`) or the stub path otherwise.
//! The old fixture string from the stub has been replaced with an explicit
//! call through `WhisperAsr` so that all downstream code exercises the same
//! code path regardless of feature flag.

use crate::{Consent, GaianError};
use crate::asr::{AsrConfig, AsrError, WhisperAsr};

/// Maps `AsrError` into the canonical `GaianError` surface so callers only
/// need to handle one error type.
impl From<AsrError> for GaianError {
    fn from(e: AsrError) -> Self {
        match e {
            AsrError::FeatureDisabled | AsrError::ModelLoad(_) | AsrError::InferenceFailed(_) => {
                // Surface as NoConsent so the UI can show a human-readable
                // message rather than a raw internal error.
                GaianError::NoConsent
            }
        }
    }
}

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

    /// Transcribe 16 kHz mono PCM `samples` to text locally via whisper.cpp.
    ///
    /// Requires `self.ready == true` and `self.local_only == true`
    /// (GAIAN always runs inference on-device).
    ///
    /// Returns the transcribed string.  When the `whisper` feature is OFF the
    /// result is an empty string (see `asr::WhisperAsr::transcribe`).
    pub fn transcribe(
        &self,
        samples: &[f32],
        cfg: AsrConfig,
    ) -> Result<String, GaianError> {
        if !self.local_only || !self.ready {
            return Err(GaianError::NoConsent);
        }
        let mut asr = WhisperAsr::new(cfg).map_err(GaianError::from)?;
        asr.transcribe(samples).map_err(GaianError::from)
    }

    /// Synthesise speech from `sentence` (TTS stub — Kokoro/Piper not yet wired).
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

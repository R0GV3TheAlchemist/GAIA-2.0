//! #71 Digital Me vault view. Not Ollama, Graphiti, or TwinVoice.

use crate::{Consent, GaianError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowKind {
    People,
    Places,
    Preferences,
}

#[derive(Debug, Default)]
pub struct DigitalVault {
    facts: Vec<(KnowKind, String)>,
    corrections: Vec<String>,
    learning: bool,
}

impl DigitalVault {
    pub fn talk_offline(consent: &Consent, text: &str) -> Result<String, GaianError> {
        if !consent.self_consent || !consent.subject_is_self {
            return Err(GaianError::NoConsent);
        }
        Ok(format!("offline:{text}"))
    }

    pub fn store(&mut self, kind: KnowKind, fact: &str) {
        self.facts.push((kind, fact.into()));
    }

    pub fn show_what_you_know(&self) -> &[(KnowKind, String)] {
        &self.facts
    }

    pub fn correct(&mut self, consent: &Consent, note: &str) -> Result<(), GaianError> {
        if consent.age_years < 16 {
            return Err(GaianError::Under16);
        }
        self.corrections.push(note.into());
        Ok(())
    }

    pub fn enable_learning(&mut self, consent: &Consent) -> Result<(), GaianError> {
        if consent.age_years < 16 {
            return Err(GaianError::Under16);
        }
        self.learning = true;
        Ok(())
    }

    pub fn wipe(&mut self) {
        self.facts.clear();
        self.corrections.clear();
        self.learning = false;
    }
}

//! #61 Digital Me stub. Not Ollama, Graphiti, or TwinVoice.

use crate::{Consent, GaianError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryTier {
    Sensory,
    Working,
    Episodic,
    Semantic,
    Procedural,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Persona {
    pub openness: u8,
    pub conscientiousness: u8,
    pub extraversion: u8,
    pub agreeableness: u8,
    pub neuroticism: u8,
    pub values: String,
    pub learning_on: bool,
}

impl Persona {
    pub fn editable(consent: &Consent) -> Result<Self, GaianError> {
        if consent.age_years < 16 {
            return Err(GaianError::BehavioralLearningBlocked);
        }
        Ok(Self {
            openness: 50,
            conscientiousness: 50,
            extraversion: 50,
            agreeableness: 50,
            neuroticism: 50,
            values: "user-editable fixture".into(),
            learning_on: false,
        })
    }

    pub fn enable_learning(&mut self, consent: &Consent) -> Result<(), GaianError> {
        if consent.age_years < 16 {
            return Err(GaianError::BehavioralLearningBlocked);
        }
        self.learning_on = true;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct PersonalMemory {
    notes: Vec<(MemoryTier, String)>,
}

impl PersonalMemory {
    pub fn put(&mut self, tier: MemoryTier, note: &str) {
        self.notes.push((tier, note.into()));
    }

    pub fn inspect(&self) -> &[(MemoryTier, String)] {
        &self.notes
    }

    pub fn wipe(&mut self) {
        self.notes.clear();
    }
}

pub struct DigitalMe;

impl DigitalMe {
    pub fn ask(consent: &Consent, text: &str) -> Result<String, GaianError> {
        if !consent.self_consent || !consent.subject_is_self {
            return Err(GaianError::NoConsent);
        }
        if consent.age_years < 16 {
            return Err(GaianError::BehavioralLearningBlocked);
        }
        Ok(format!("offline fixture reply: {text}"))
    }
}

pub fn g2g_send(signed: bool, scope: &str) -> Result<(), GaianError> {
    if !signed {
        return Err(GaianError::Unsigned);
    }
    if scope.trim().is_empty() {
        return Err(GaianError::NoConsent);
    }
    Ok(())
}

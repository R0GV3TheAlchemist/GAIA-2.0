//! #111 / #119 session assessment. Not MediaPipe, Whisper, or Open Badges live.

use crate::SkillError;

#[derive(Debug, Default)]
pub struct Session {
    pub active: bool,
    pub camera: bool,
    pub mic: bool,
    evidence: Vec<String>,
}

impl Session {
    pub fn start(owner_started: bool) -> Result<Self, SkillError> {
        if !owner_started {
            return Err(SkillError::AmbientDenied);
        }
        Ok(Self {
            active: true,
            camera: true,
            mic: true,
            evidence: vec![],
        })
    }

    pub fn end(&mut self) {
        self.active = false;
        self.camera = false;
        self.mic = false;
    }

    pub fn record(&mut self, item: &str) {
        self.evidence.push(item.into());
    }

    pub fn withdraw(&mut self) {
        self.evidence.clear();
        self.end();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Badge {
    pub skill_id: String,
    pub level: String,
    pub issuer: String,
    pub owner_key: String,
}

impl Badge {
    pub fn mint(skill_id: &str, clinical: bool) -> Result<Self, SkillError> {
        if clinical {
            return Err(SkillError::ClinicalCert);
        }
        Ok(Self {
            skill_id: skill_id.into(),
            level: "novice".into(),
            issuer: "gaia-skills-fixture".into(),
            owner_key: "owner-local".into(),
        })
    }
}

pub fn hidden_profile_api() -> bool {
    false
}

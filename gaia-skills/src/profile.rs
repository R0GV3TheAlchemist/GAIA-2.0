//! #110 on-device skill profile. No ambient scoring. No child EI inference.

use crate::SkillError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillProfile {
    pub sync: bool,
    pub inferred_se: bool,
    pub strengths: Vec<String>,
    pub gaps: Vec<String>,
    pub goals: Vec<String>,
}

impl SkillProfile {
    pub fn local() -> Self {
        Self {
            sync: false,
            inferred_se: false,
            strengths: vec![],
            gaps: vec![],
            goals: vec![],
        }
    }

    pub fn enable_inferred_se(&mut self, age_years: u8) -> Result<(), SkillError> {
        if age_years < 16 {
            return Err(SkillError::ChildEiBlocked);
        }
        self.inferred_se = true;
        Ok(())
    }

    pub fn enable_sync(&mut self, opt_in: bool) -> Result<(), SkillError> {
        if !opt_in {
            return Err(SkillError::SyncDenied);
        }
        self.sync = true;
        Ok(())
    }
}

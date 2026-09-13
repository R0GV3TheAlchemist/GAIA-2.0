//! #117 local vault profile CRUD. Clusters are an explicit toggle.

use crate::{SkillError, SkillProfile};

#[derive(Debug, Default)]
pub struct VaultProfile {
    pub profile: SkillProfile,
    pub clusters: bool,
}

impl Default for SkillProfile {
    fn default() -> Self {
        Self::local()
    }
}

impl VaultProfile {
    pub fn offline() -> Self {
        Self {
            profile: SkillProfile::local(),
            clusters: false,
        }
    }

    pub fn set_goal(&mut self, goal: &str) {
        self.profile.goals.clear();
        self.profile.goals.push(goal.into());
    }

    pub fn toggle_clusters(&mut self, on: bool, age_years: u8) -> Result<(), SkillError> {
        if on && age_years < 16 {
            return Err(SkillError::ChildEiBlocked);
        }
        self.clusters = on;
        Ok(())
    }

    pub fn delete(&mut self) {
        self.profile = SkillProfile::local();
        self.clusters = false;
    }
}

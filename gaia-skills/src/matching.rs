//! #112 / #120 opt-in cards. TEK sealed. No marketplace ranking of children.

use crate::SkillError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillCard {
    pub published: bool,
    pub age_years: u8,
    pub skill: String,
}

impl SkillCard {
    pub fn search(cards: &[SkillCard]) -> Vec<&SkillCard> {
        cards.iter().filter(|c| c.published).collect()
    }

    pub fn rank_in_marketplace(&self) -> Result<(), SkillError> {
        if self.age_years < 18 {
            return Err(SkillError::ChildRank);
        }
        if !self.published {
            return Err(SkillError::Unpublished);
        }
        Ok(())
    }
}

pub fn tek_skill(grant: bool) -> Result<&'static str, SkillError> {
    if !grant {
        return Err(SkillError::NoGrant);
    }
    Ok("tek-skill-granted")
}

pub fn skills_v1_tagged() -> bool {
    false
}

pub fn global_profile_dump() -> bool {
    false
}

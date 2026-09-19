//! #122 skill card. Unmeasured is never Level 5.

use crate::{banned_level6, AisdError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Maturity {
    L1 = 1,
    L2 = 2,
    L3 = 3,
    L4 = 4,
    L5 = 5,
    L6 = 6,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiSkill {
    pub id: String,
    pub measured: bool,
    pub maturity: Option<Maturity>,
    pub limitations: Vec<String>,
}

impl AiSkill {
    pub fn unmeasured(id: &str) -> Self {
        Self {
            id: id.into(),
            measured: false,
            maturity: None,
            limitations: vec!["unmeasured".into()],
        }
    }
}

pub fn assign_maturity(
    domain: &str,
    level: Maturity,
    measured: bool,
) -> Result<Maturity, AisdError> {
    if !measured && level >= Maturity::L5 {
        return Err(AisdError::Unmeasured);
    }
    if level == Maturity::L6 && banned_level6().contains(&domain) {
        return Err(AisdError::Level6Banned);
    }
    Ok(level)
}

pub fn protein_structure() -> AiSkill {
    AiSkill {
        id: "aisd:science:protein-structure-prediction".into(),
        measured: false,
        maturity: None,
        limitations: vec!["license-check", "unmeasured"]
            .into_iter()
            .map(str::to_string)
            .collect(),
    }
}

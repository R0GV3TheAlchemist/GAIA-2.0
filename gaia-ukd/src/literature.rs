//! #89 literature/skill stubs and local knowledge state updates.

use crate::{KnowledgeLevel, UkdError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LicensedNode {
    pub kind: String,
    pub title: String,
    pub license: String,
}

impl LicensedNode {
    pub fn paper() -> Self {
        Self {
            kind: "paper".into(),
            title: "fixture-open-paper".into(),
            license: "CC-BY-4.0".into(),
        }
    }

    pub fn skill() -> Self {
        Self {
            kind: "skill".into(),
            title: "fixture-open-skill".into(),
            license: "CC-BY-SA-4.0".into(),
        }
    }
}

pub fn present(concept: &str, level: KnowledgeLevel) -> String {
    format!("{concept}@{level:?}")
}

#[derive(Debug, Default)]
pub struct KnowledgeState {
    pub known: Vec<String>,
    pub learning: Vec<String>,
    pub frontier: Vec<String>,
    pub gaps: Vec<String>,
    pub strengths: Vec<String>,
    pub interests: Vec<String>,
    pub sync: bool,
}

impl KnowledgeState {
    pub fn local() -> Self {
        Self {
            sync: false,
            ..Self::default()
        }
    }

    pub fn mark_learned(&mut self, step: &str) -> Result<(), UkdError> {
        if step.is_empty() {
            return Err(UkdError::UnknownNode);
        }
        self.learning.retain(|s| s != step);
        if !self.known.iter().any(|s| s == step) {
            self.known.push(step.into());
        }
        Ok(())
    }
}

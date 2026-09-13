//! #80 TEK empty-by-default and local GAIAN knowledge state.

use crate::UkdError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowledgeLevel {
    Sprout,
    Grow,
    Branch,
    Crown,
    Master,
}

#[derive(Debug, Default)]
pub struct TekStore {
    pub public: Vec<String>,
}

impl TekStore {
    pub fn new() -> Self {
        Self { public: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.public.is_empty()
    }
}

pub fn publish_tek(agreement: bool, collection: &str) -> Result<(), UkdError> {
    if collection.trim().is_empty() {
        return Err(UkdError::Uncited);
    }
    if !agreement {
        return Err(UkdError::NoAgreement);
    }
    Ok(())
}

#[derive(Debug, Default)]
pub struct GaianKnowledge {
    pub known: Vec<String>,
    pub learning: Vec<String>,
    pub frontier: Vec<String>,
    pub gaps: Vec<String>,
}

impl GaianKnowledge {
    pub fn local_only() -> Self {
        Self::default()
    }
}

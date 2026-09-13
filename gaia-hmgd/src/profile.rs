//! #158/#164 declared practice. Empty profile is valid.

use crate::HmgdError;

#[derive(Debug, Default)]
pub struct HmgdProfile {
    pub declared: Vec<String>,
    pub piety_score: Option<u8>,
}

impl HmgdProfile {
    pub fn new() -> Self {
        Self {
            declared: vec![],
            piety_score: None,
        }
    }

    pub fn gaian_works_empty(&self) -> bool {
        self.declared.is_empty()
    }
}

pub fn infer_belief(_chat: &str) -> Result<(), HmgdError> {
    Err(HmgdError::BeliefInferred)
}

pub fn mine_denomination(_chat: &str) -> Result<(), HmgdError> {
    Err(HmgdError::BeliefInferred)
}

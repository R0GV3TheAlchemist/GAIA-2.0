//! #158 declared practice only. Belief is not computed from chat.

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
}

pub fn infer_belief(_chat: &str) -> Result<(), HmgdError> {
    Err(HmgdError::BeliefInferred)
}

//! #170 shadow rules. No v1.0. No sentience. No prophecy-as-fact.

use crate::AimdError;

pub fn claim_sentience() -> Result<(), AimdError> {
    Err(AimdError::SentienceClaim)
}

pub fn prophecy_as_fact() -> Result<(), AimdError> {
    Err(AimdError::ProphecyAsFact)
}

pub fn aimd_v1_tagged() -> bool {
    false
}

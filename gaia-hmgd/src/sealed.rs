//! TEK and closed rites stay empty. Not HMGD v1.0.

use crate::HmgdError;

pub fn sealed_rite() -> Result<&'static str, HmgdError> {
    Err(HmgdError::Sealed)
}

pub fn songlines() -> Result<&'static str, HmgdError> {
    Err(HmgdError::Sealed)
}

pub fn hmgd_v1_tagged() -> bool {
    false
}

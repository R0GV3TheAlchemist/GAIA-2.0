//! #211 prompts + passports. Access score required. No proprietary ledger.

use crate::SaError;

#[derive(Debug)]
pub struct Plan {
    pub access: Option<u8>,
}

pub fn generate(access: Option<u8>) -> Result<Plan, SaError> {
    if access.is_none() {
        return Err(SaError::AccessSkipped);
    }
    Ok(Plan { access })
}

pub fn passport() -> &'static str {
    "json-no-proprietary-ledger"
}

pub fn reuse_first() -> bool {
    true
}

//! #131 dispatch log. Not live orchestrator. Not AISD v1.0.

use crate::{AisdError, Maturity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallLog {
    pub skill_id: String,
    pub maturity: Maturity,
}

pub fn dispatch(skill_id: Option<&str>, maturity: Maturity) -> Result<CallLog, AisdError> {
    let id = skill_id.ok_or(AisdError::InsufficientMaturity)?;
    Ok(CallLog {
        skill_id: id.into(),
        maturity,
    })
}

pub fn capability_copy(text: &str) -> Result<(), AisdError> {
    if text.to_ascii_lowercase().contains("autonomous agi") {
        return Err(AisdError::Unmeasured);
    }
    Ok(())
}

pub fn release_gaps() -> [&'static str; 3] {
    ["swe-pro-engineering", "calibration", "no AISD v1.0"]
}

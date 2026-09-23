//! #182/#189 advise only. No dam/grid flip. No SI v1.0.

use crate::SiError;

pub fn advise(action: &str, ticket: bool) -> Result<&'static str, SiError> {
    if (action.contains("dam") || action.contains("grid")) && !ticket {
        return Err(SiError::NeedsTicket);
    }
    if action.contains("conscious") {
        return Err(SiError::ConsciousMarketing);
    }
    Ok("advisory")
}

pub fn si_v1_tagged() -> bool {
    false
}

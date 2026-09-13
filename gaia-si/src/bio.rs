//! #181/#188 nature-first. No pathogen protocol.

use crate::SiError;

pub fn pathogen_protocol() -> Result<(), SiError> {
    Err(SiError::PathogenProtocol)
}

pub fn review_field(named_review: bool) -> Result<(), SiError> {
    if !named_review {
        return Err(SiError::NeedsReview);
    }
    Ok(())
}

pub fn nature_first() -> [&'static str; 2] {
    ["wetlands", "urban-canopy"]
}

//! #174 chips. Invention cannot be Tier 1.

use crate::AimdError;

pub fn wonder_mode(opt_in: bool) -> &'static str {
    if opt_in {
        "label-only"
    } else {
        "off"
    }
}

pub fn chip(cited: bool, hazard: bool) -> &'static str {
    if hazard {
        "hazard-blocked"
    } else if cited {
        "verified"
    } else {
        "invention"
    }
}

pub fn tier1(chip: &str) -> Result<(), AimdError> {
    if chip == "invention" {
        return Err(AimdError::TierOneInvention);
    }
    Ok(())
}

pub fn consciousness_qa() -> &'static str {
    "agnostic: GAIA does not claim sentience"
}

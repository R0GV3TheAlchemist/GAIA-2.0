//! #135/#141 vault profile. Declare, do not infer DNA.

use crate::HspdError;

#[derive(Debug, Default)]
pub struct HspdProfile {
    pub genetic_indicators: Vec<String>,
    pub dna_required: bool,
    pub declared: Vec<String>,
}

impl HspdProfile {
    pub fn new() -> Self {
        Self {
            genetic_indicators: vec![],
            dna_required: false,
            declared: vec![],
        }
    }

    pub fn infer_actn3(_raw: &str) -> Result<(), HspdError> {
        Err(HspdError::ChildGenetic)
    }

    pub fn child_genetic(age: u8) -> Result<(), HspdError> {
        if age < 18 {
            return Err(HspdError::ChildGenetic);
        }
        Ok(())
    }

    pub fn child_tag(age: u8, tag: &str) -> Result<(), HspdError> {
        if age < 16 && (tag.contains("surgical") || tag.contains("pharmacologic")) {
            return Err(HspdError::ChildTag);
        }
        Ok(())
    }
}

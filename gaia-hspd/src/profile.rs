//! #135 profile shape. No silent genetics. Children blocked.

use crate::HspdError;

#[derive(Debug, Default)]
pub struct HspdProfile {
    pub genetic_indicators: Vec<String>,
    pub dna_required: bool,
}

impl HspdProfile {
    pub fn new() -> Self {
        Self {
            genetic_indicators: vec![],
            dna_required: false,
        }
    }

    pub fn child_genetic(age: u8) -> Result<(), HspdError> {
        if age < 18 {
            return Err(HspdError::ChildGenetic);
        }
        Ok(())
    }
}

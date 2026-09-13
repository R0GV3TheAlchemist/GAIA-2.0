//! #104 science + professional packs. Not GraphCast or a license to practice.

use crate::AikdError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScienceAnswer {
    pub text: String,
    pub earth_twin_product: String,
}

impl ScienceAnswer {
    pub fn with_product(product_id: &str) -> Result<Self, AikdError> {
        if product_id.is_empty() {
            return Err(AikdError::MissingCitation);
        }
        Ok(Self {
            text: "science-fixture".into(),
            earth_twin_product: product_id.into(),
        })
    }
}

pub fn professional_disclaimer(kind: &str) -> String {
    format!("{kind}: not professional advice")
}

pub fn gaia_certifies_usmle_or_bar() -> bool {
    false
}

pub fn insurer_automation() -> Result<(), AikdError> {
    Err(AikdError::ClosedScoreClaim)
}

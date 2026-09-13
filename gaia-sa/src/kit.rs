//! #204/#210 nature-first kit. Zero live-culture default.

use crate::SaError;

pub fn palette() -> [&'static str; 4] {
    ["timber", "bamboo", "hemp", "mycelium-catalog"]
}

pub fn live_culture() -> Result<(), SaError> {
    Err(SaError::LiveCulture)
}

pub fn carbon_claim(factor: Option<f64>) -> Result<(), SaError> {
    if factor.is_none() {
        return Err(SaError::SloganCarbon);
    }
    Ok(())
}

//! #130 recommender. Fake Level 5 is refused.

use crate::{AisdError, Maturity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rec {
    pub family: String,
    pub gaia_measured: bool,
}

pub fn recommend(min: Maturity, have: Option<Maturity>) -> Result<Rec, AisdError> {
    match have {
        Some(m) if m >= min => Ok(Rec {
            family: "language".into(),
            gaia_measured: true,
        }),
        _ => Err(AisdError::InsufficientMaturity),
    }
}

pub fn measured_families() -> [&'static str; 3] {
    ["language", "code", "safety"]
}

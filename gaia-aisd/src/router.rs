//! #125 routing notes. Not AISD v1.0. High-stakes cannot pick L6.

use crate::{AisdError, Maturity};

pub fn high_stakes_level6() -> Result<(), AisdError> {
    Err(AisdError::HighStakesLevel6)
}

pub fn ask_aisd(claimed: Maturity, actual: Maturity) -> Result<(), AisdError> {
    if claimed >= Maturity::L5 && actual <= Maturity::L2 {
        return Err(AisdError::InsufficientMaturity);
    }
    Ok(())
}

pub fn aisd_v1_tagged() -> bool {
    false
}

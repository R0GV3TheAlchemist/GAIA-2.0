//! #163 links. Climate is not spirits by default.

use crate::HmgdError;

pub fn climate_as_spirit() -> Result<(), HmgdError> {
    Err(HmgdError::ClimateRewrite)
}

pub fn sacred_layer(opt_in: bool) -> Result<(), HmgdError> {
    if !opt_in {
        return Err(HmgdError::NotOptIn);
    }
    Ok(())
}

//! #180 allow-listed actuation. Unauthorized denied. Kill switch.

use crate::SiError;

pub fn actuate(authorized: bool) -> Result<(), SiError> {
    if !authorized {
        return Err(SiError::ActuatorDenied);
    }
    Ok(())
}

pub fn kill() -> bool {
    true
}

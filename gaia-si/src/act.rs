//! #180/#187 HITL. Life-safety needs ack.

use crate::SiError;

pub fn actuate(authorized: bool) -> Result<(), SiError> {
    if !authorized {
        return Err(SiError::ActuatorDenied);
    }
    Ok(())
}

pub fn life_safety(human_ack: bool) -> Result<(), SiError> {
    if !human_ack {
        return Err(SiError::ActuatorDenied);
    }
    Ok(())
}

pub fn kill() -> bool {
    true
}

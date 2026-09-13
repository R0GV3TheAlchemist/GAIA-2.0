//! #165 opt-in rooms. Outsiders not enrolled. No v1.0.

use crate::HmgdError;

#[derive(Debug, Clone)]
pub struct Room {
    pub enrolled: bool,
}

pub fn join_room(opt_in: bool) -> Result<Room, HmgdError> {
    if !opt_in {
        return Ok(Room { enrolled: false });
    }
    Ok(Room { enrolled: true })
}

pub fn sell_closed_rite() -> Result<(), HmgdError> {
    Err(HmgdError::SaleForbidden)
}

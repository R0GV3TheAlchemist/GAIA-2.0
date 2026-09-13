//! #142 practice sessions. Devices stop. Groups are revocable.

use crate::HspdError;

#[derive(Debug, Clone)]
pub struct Session {
    pub live: bool,
    pub listed: bool,
}

impl Session {
    pub fn start() -> Self {
        Self {
            live: true,
            listed: true,
        }
    }
}

pub fn end_session(s: &mut Session) {
    s.live = false;
}

pub fn join_group(opt_in: bool) -> Result<Session, HspdError> {
    if !opt_in {
        return Err(HspdError::AlwaysOn);
    }
    Ok(Session {
        live: true,
        listed: false,
    })
}

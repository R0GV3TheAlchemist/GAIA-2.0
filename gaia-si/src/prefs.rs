//! #184 read-only fixture twin + prefs. No HVAC write.

use crate::SiError;

#[derive(Debug, Default)]
pub struct Prefs {
    pub comfort: String,
}

impl Prefs {
    pub fn wipe(self) -> Self {
        Self::default()
    }
}

pub fn hvac_write() -> Result<(), SiError> {
    Err(SiError::ActuatorDenied)
}

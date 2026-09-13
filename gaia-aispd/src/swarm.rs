//! #147 bounded swarm. RSI default deny.

use crate::AispdError;

#[derive(Debug, Clone)]
pub struct Swarm {
    pub size: u8,
    pub logged: bool,
}

impl Swarm {
    pub fn default_bound() -> Self {
        Self {
            size: 3,
            logged: true,
        }
    }
}

pub fn start_rsi() -> Result<(), AispdError> {
    Err(AispdError::Prohibited)
}

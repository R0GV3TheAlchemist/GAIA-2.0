//! #147/#153 bounded swarm. RSI default deny. Owner required.

use crate::AispdError;

#[derive(Debug, Clone)]
pub struct Swarm {
    pub size: u8,
    pub logged: bool,
    pub owner: String,
}

impl Swarm {
    pub fn default_bound() -> Self {
        Self {
            size: 3,
            logged: true,
            owner: "fixture-owner".into(),
        }
    }
}

pub fn start(owner: Option<&str>) -> Result<Swarm, AispdError> {
    let owner = owner.filter(|s| !s.is_empty()).ok_or(AispdError::NoOwner)?;
    Ok(Swarm {
        size: 3,
        logged: true,
        owner: owner.into(),
    })
}

pub fn start_rsi() -> Result<(), AispdError> {
    Err(AispdError::Prohibited)
}

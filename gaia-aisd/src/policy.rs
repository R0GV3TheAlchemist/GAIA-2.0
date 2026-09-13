//! #129 component policy. Agents need safety ≥ L3 before tools.

use crate::{AisdError, Maturity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    EarthTwin,
    Gaian,
    Ukd,
    Agents,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentPolicy {
    pub component: Component,
    pub min_safety: Maturity,
}

impl ComponentPolicy {
    pub fn agents() -> Self {
        Self {
            component: Component::Agents,
            min_safety: Maturity::L3,
        }
    }
}

pub fn allow(task: &str, maturity: Maturity) -> Result<(), AisdError> {
    if task.contains("multi-day") && maturity <= Maturity::L2 {
        return Err(AisdError::InsufficientMaturity);
    }
    Ok(())
}

//! #149 schema. Failures required. ASI cannot be active.

use crate::AispdError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Emerging,
    FutureMonitor,
    ProhibitedToImplement,
}

#[derive(Debug, Clone)]
pub struct AispdNode {
    pub id: String,
    pub status: Status,
    pub failures: Vec<String>,
}

pub fn parse_node(id: &str, status: Status, failures: &[&str]) -> Result<AispdNode, AispdError> {
    if failures.is_empty() {
        return Err(AispdError::MissingFailures);
    }
    if id.contains("asi") && status == Status::Active {
        return Err(AispdError::AsiActive);
    }
    if id.contains("rsi") && status == Status::Active {
        return Err(AispdError::AsiActive);
    }
    Ok(AispdNode {
        id: id.into(),
        status,
        failures: failures.iter().map(|s| s.to_string()).collect(),
    })
}

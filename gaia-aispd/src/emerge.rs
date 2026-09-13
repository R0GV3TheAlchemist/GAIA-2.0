//! #152 emergence/deception watch. Not a wontfix feature.

use crate::AispdError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Finding {
    EmergentUnexpected,
    Deception,
}

pub fn label_trace(kind: &str) -> Finding {
    if kind.contains("decept") {
        Finding::Deception
    } else {
        Finding::EmergentUnexpected
    }
}

pub fn close_finding(as_wontfix: bool) -> Result<(), AispdError> {
    if as_wontfix {
        return Err(AispdError::WontfixBlocked);
    }
    Ok(())
}

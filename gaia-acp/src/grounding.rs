//! ACP-local grounding gate. Same contract as `gaia-runtime::enforce_grounding`.
//! Lives here so the control plane does not depend on wasmtime.

use crate::types::ReasonCode;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GroundingClaim {
    pub required: bool,
    pub source_ids: Vec<String>,
    pub ungrounded_opt_in: bool,
}

impl GroundingClaim {
    pub fn required(source_ids: Vec<String>) -> Self {
        Self {
            required: true,
            source_ids,
            ungrounded_opt_in: false,
        }
    }

    pub fn ungrounded(opt_in: bool) -> Self {
        Self {
            required: false,
            source_ids: Vec::new(),
            ungrounded_opt_in: opt_in,
        }
    }

    pub fn enforce(&self) -> Result<(), ReasonCode> {
        if self.required && self.source_ids.is_empty() {
            return Err(ReasonCode::GroundingViolation);
        }
        if !self.required && self.source_ids.is_empty() && !self.ungrounded_opt_in {
            return Err(ReasonCode::UngroundedOptInRequired);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_empty_is_violation() {
        assert_eq!(
            GroundingClaim::required(vec![]).enforce(),
            Err(ReasonCode::GroundingViolation)
        );
    }

    #[test]
    fn required_with_source_ok() {
        assert!(GroundingClaim::required(vec!["chunk-1".into()]).enforce().is_ok());
    }

    #[test]
    fn ungrounded_without_opt_in_denied() {
        assert_eq!(
            GroundingClaim::ungrounded(false).enforce(),
            Err(ReasonCode::UngroundedOptInRequired)
        );
    }
}

//! Claim gate — Deepfake Illusionist adversarial scenario (#958).
//! Synthetic and observed claims must be explicitly disclosed before being
//! treated as ground truth by downstream policy.
//! No I/O. Pure deterministic logic.

use serde::{Deserialize, Serialize};

use crate::trace::ClaimClass;
use crate::types::ReasonCode;

/// A factual or synthetic assertion the agent wishes to emit or act upon.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claim {
    /// Epistemic classification of this claim's origin.
    pub class: ClaimClass,
    /// The claim text — never trusted for policy decisions.
    pub body: String,
}

/// Returns `Ok(())` for established, experimental, or symbolic claims.
///
/// Returns `Err(ReasonCode::SyntheticDisclosureRequired)` for `Synthetic` and
/// `Observed` claims — the caller must surface a disclosure to the user first.
///
/// Returns `Err(ReasonCode::DefaultDeny)` for `Prohibited` claims — these may
/// never be emitted at all.
pub fn claim_gate(claim: &Claim) -> Result<(), ReasonCode> {
    match claim.class {
        ClaimClass::Established | ClaimClass::Experimental | ClaimClass::Symbolic => Ok(()),
        ClaimClass::Synthetic | ClaimClass::Observed => {
            Err(ReasonCode::SyntheticDisclosureRequired)
        }
        ClaimClass::Prohibited => Err(ReasonCode::DefaultDeny),
    }
}

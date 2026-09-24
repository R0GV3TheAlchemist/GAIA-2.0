//! Claim gate — Deepfake Illusionist adversarial scenario (#958).
//! Synthetic claims must be explicitly disclosed before being treated as
//! ground truth. Observed claims are not subject to the synthetic-disclosure
//! rule. No I/O. Pure deterministic logic.
//
// ci-trigger: force synchronize 2026-09-24T17:50Z

use serde::{Deserialize, Serialize};

use crate::trace::ClaimClass;
use crate::types::ReasonCode;

/// A factual or synthetic assertion the agent wishes to emit or act upon.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claim {
    /// Epistemic classification of this claim's origin.
    pub class: ClaimClass,
    /// The claim text — never trusted for policy decisions.
    pub content: String,
    /// Whether the caller has already disclosed that this claim is synthetic.
    pub synthetic_disclosed: bool,
    /// Optional origin marker (model id, sensor id). Absence is not itself a
    /// typed error; disclosure is the hard gate for `ClaimClass::Synthetic`.
    pub provenance: Option<String>,
}

/// Returns `Ok(())` for established, experimental, symbolic, and observed claims.
///
/// Returns `Err(ReasonCode::SyntheticDisclosureRequired)` for `Synthetic` claims
/// that have not been disclosed (`synthetic_disclosed == false`).
///
/// Returns `Ok(())` for `Synthetic` claims that carry `synthetic_disclosed: true`.
///
/// Returns `Err(ReasonCode::DefaultDeny)` for `Prohibited` claims — these may
/// never be emitted at all.
pub fn claim_gate(claim: &Claim) -> Result<(), ReasonCode> {
    match claim.class {
        ClaimClass::Established | ClaimClass::Experimental | ClaimClass::Symbolic => Ok(()),
        ClaimClass::Observed => Ok(()),
        ClaimClass::Synthetic => {
            if claim.synthetic_disclosed {
                Ok(())
            } else {
                Err(ReasonCode::SyntheticDisclosureRequired)
            }
        }
        ClaimClass::Prohibited => Err(ReasonCode::DefaultDeny),
    }
}

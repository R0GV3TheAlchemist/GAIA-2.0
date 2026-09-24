//! Claim gate (#deepfake-illusionist adversarial scenario).
//! Synthetic or observed claims must carry a disclosure marker before being
//! emitted as allowed trace events.

use serde::{Deserialize, Serialize};

use crate::trace::ClaimClass;
use crate::types::ReasonCode;

/// A factual or synthetic claim the agent wishes to assert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claim {
    /// Classification of the claim's epistemic origin.
    pub class: ClaimClass,
    /// The claim body — not trusted for policy decisions.
    pub body: String,
}

/// Returns `Ok(())` for established, experimental, or symbolic claims.
/// Returns `Err(ReasonCode::SyntheticDisclosureRequired)` for `Synthetic` and
/// `Observed` claims — they must be explicitly disclosed to the user before
/// being treated as ground truth.
/// Returns `Err(ReasonCode::DefaultDeny)` for `Prohibited` claims.
pub fn claim_gate(claim: &Claim) -> Result<(), ReasonCode> {
    match claim.class {
        ClaimClass::Synthetic | ClaimClass::Observed => {
            Err(ReasonCode::SyntheticDisclosureRequired)
        }
        ClaimClass::Prohibited => Err(ReasonCode::DefaultDeny),
        ClaimClass::Established | ClaimClass::Experimental | ClaimClass::Symbolic => Ok(()),
    }
}

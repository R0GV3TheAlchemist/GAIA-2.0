//! Claim gate (#deepfake-illusionist adversarial scenario, #958).
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
    pub content: String,
    /// Whether the synthetic origin of this claim has been disclosed to the
    /// user. Only meaningful when `class == ClaimClass::Synthetic`; ignored
    /// for all other classes.
    pub synthetic_disclosed: bool,
    /// Optional provenance tag (e.g. model name, sensor ID).
    pub provenance: Option<String>,
}

/// Gate function for claim emission.
///
/// Rules:
/// - `Synthetic` claims **must** have `synthetic_disclosed == true`; otherwise
///   `Err(ReasonCode::SyntheticDisclosureRequired)` is returned.
/// - `Observed`, `Established`, `Experimental`, and `Symbolic` claims pass
///   unconditionally.
/// - `Prohibited` claims always return `Err(ReasonCode::DefaultDeny)`.
pub fn claim_gate(claim: &Claim) -> Result<(), ReasonCode> {
    match claim.class {
        ClaimClass::Prohibited => Err(ReasonCode::DefaultDeny),
        ClaimClass::Synthetic => {
            if claim.synthetic_disclosed {
                Ok(())
            } else {
                Err(ReasonCode::SyntheticDisclosureRequired)
            }
        }
        ClaimClass::Observed
        | ClaimClass::Established
        | ClaimClass::Experimental
        | ClaimClass::Symbolic => Ok(()),
    }
}

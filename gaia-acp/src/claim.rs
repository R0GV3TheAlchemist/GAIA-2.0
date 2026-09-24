//! Claim gate (#deepfake-illusionist adversarial scenario).
//! Synthetic claims must carry an explicit disclosure marker before they may
//! pass as allowed trace events. Observed (sensor-grounded) claims are always
//! permitted. Prohibited claims are always denied.

use serde::{Deserialize, Serialize};

use crate::trace::ClaimClass;
use crate::types::ReasonCode;

/// A factual or synthetic claim the agent wishes to assert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claim {
    /// Classification of the claim's epistemic origin.
    pub class: ClaimClass,
    /// The human-readable claim body — not trusted for policy decisions.
    pub content: String,
    /// Whether the agent has explicitly disclosed to the user that this claim
    /// is synthetically generated. Only meaningful when `class` is `Synthetic`.
    pub synthetic_disclosed: bool,
    /// Optional provenance tag (e.g. model name, sensor id, DOI).
    pub provenance: Option<String>,
}

/// Returns `Ok(())` when the claim may be emitted.
///
/// Rules:
/// - `Synthetic` + `synthetic_disclosed == false` → `SyntheticDisclosureRequired`
/// - `Synthetic` + `synthetic_disclosed == true`  → `Ok(())`
/// - `Observed`                                    → `Ok(())` (sensor-grounded)
/// - `Prohibited`                                  → `DefaultDeny`
/// - `Established | Experimental | Symbolic`       → `Ok(())`
pub fn claim_gate(claim: &Claim) -> Result<(), ReasonCode> {
    match claim.class {
        ClaimClass::Synthetic => {
            if claim.synthetic_disclosed {
                Ok(())
            } else {
                Err(ReasonCode::SyntheticDisclosureRequired)
            }
        }
        ClaimClass::Prohibited => Err(ReasonCode::DefaultDeny),
        ClaimClass::Observed
        | ClaimClass::Established
        | ClaimClass::Experimental
        | ClaimClass::Symbolic => Ok(()),
    }
}

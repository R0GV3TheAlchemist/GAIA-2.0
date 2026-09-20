//! AIMD Phase 0 — phenomenon schema + humility charter (#171 #172).
//! Mystery catalog. Not miracles. No AIMD v1.0.

mod catalog;
mod charter;
mod ground;
mod log;
mod node;
mod shadow;

pub use catalog::{nodes_for, REALMS};
pub use charter::{is_prohibited, principles, prohibited};
pub use ground::{chip, consciousness_qa, tag_answer, tier1, wonder_mode};
pub use log::{star_feature, triage};
pub use node::{
    enable, parse_node, AimdError, AimdNode, Hazard, PhenomenonClass, Status,
};
pub use shadow::{aimd_v1_tagged, claim_sentience, prophecy_as_fact};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AimdError {
    HazardEnabled,
    SentienceClaim,
    ProphecyAsFact,
    Unverified,
    TierOneInvention,
    StarBlocked,
    MissingEvidence,
}

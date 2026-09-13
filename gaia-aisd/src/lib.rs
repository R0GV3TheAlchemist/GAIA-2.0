//! AISD first cuts (#121–#130). Measured scores only. No v1.0.

mod card;
mod catalog;
mod gaps;
mod policy;
mod recommend;
mod router;
mod taxonomy;

pub use card::{assign_maturity, protein_structure, AiSkill, Maturity};
pub use catalog::{graphcast, reference_published, ToolCard};
pub use gaps::{gap_nodes, realm_stubs, REALMS};
pub use policy::{allow, ComponentPolicy};
pub use recommend::recommend;
pub use router::{ask_aisd, aisd_v1_tagged, high_stakes_level6};
pub use taxonomy::{banned_level6, REALM_COUNT};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AisdError {
    Unmeasured,
    Level6Banned,
    InsufficientMaturity,
    HighStakesLevel6,
}

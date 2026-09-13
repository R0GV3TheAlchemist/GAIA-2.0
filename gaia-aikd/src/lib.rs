//! AIKD first cuts (#93–#100). Open weights default. Not a practice license.

mod agentic;
mod answer;
mod card;
mod domains;
mod meta;
mod registry;
mod taxonomy;

pub use agentic::{computer_use, world_model_claim, ToolResult};
pub use answer::{Answer, Layer};
pub use card::{cannot_know, model_card, published_closed_score_is_ours, ModelCard, Tier};
pub use domains::Adapter;
pub use meta::{aikd_v1_tagged, audit_report, knows_everything};
pub use registry::{system_tags, BenchRow};
pub use taxonomy::{cannot_know_catalog, query_cannot_know, KnowledgeLayer, KnowledgeType, Status};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AikdError {
    CannotKnow,
    NeedVerify,
    ClosedScoreClaim,
    MissingCitation,
}

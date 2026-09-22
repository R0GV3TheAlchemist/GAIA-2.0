//! AIKD first cuts (#93–#106, #621). Open weights default. Not a practice license.

mod agentic;
mod answer;
pub mod benchmark_db;
pub mod card;
mod domains;
pub mod domain;
mod eval;
mod hallucination;
mod honesty;
mod meta;
mod packs;
pub mod model_registry;
mod registry;
mod retrieve;
mod session;
mod taxonomy;
mod verify;

pub use agentic::{computer_use, world_model_claim, ToolResult};
pub use answer::{Answer, Layer};
pub use benchmark_db::{
    all_benchmarks, bench_by_id, benches_for_domain, superhuman_benches, BenchmarkEntry,
};
pub use card::{cannot_know, model_card, published_closed_score_is_ours, ModelCard, Tier};
pub use domain::{all_domains, domain_by_id, Domain};
pub use domains::Adapter;
pub use eval::{
    calibration_score, gap_detected, measured_vs_published, overconfident_fixture_dropped,
    release_ready, EvalReport,
};
pub use hallucination::{
    hallucination_warning, propagate_uncertainty, tier_floor, uncertainty_band,
    Generation, HallucinationClass, HallucinationKind, HallucinationWarning, UncertaintyBand,
};
pub use meta::{aikd_v1_tagged, audit_report, knows_everything};
pub use model_registry::{model_by_id, models_for_domain, open_models, ModelEntry};
pub use packs::{
    gaia_certifies_usmle_or_bar, insurer_automation, professional_disclaimer, ScienceAnswer,
};
pub use registry::{system_tags, BenchRow};
pub use retrieve::QueryHit;
pub use session::{embodied_enabled, sandbox_breakout, ToolCube};
pub use taxonomy::{cannot_know_catalog, query_cannot_know, KnowledgeLayer, KnowledgeType, Status};
pub use verify::Executed;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AikdError {
    CannotKnow,
    NeedVerify,
    ClosedScoreClaim,
    MissingCitation,
}

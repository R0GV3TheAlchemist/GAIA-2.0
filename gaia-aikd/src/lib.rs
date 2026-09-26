//! AIKD first cuts (#93–#106, #621, #622, #628). Open weights default. Not a practice license.
//!
//! Golden Age five lines (#955):
//! This is what I know.
//! This is what I infer.
//! This is what I experience.
//! This is what I don't know.
//! And here is what would change my mind.

mod agentic;
mod answer;
pub mod benchmark_db;
pub mod card;
mod citations;
mod cite;
pub mod domain;
mod domains;
mod eval;
mod hallucination;
mod honesty;
mod hybrid;
mod meta;
pub mod model_registry;
mod packs;
pub mod quality;
mod rank;
mod registry;
mod retrieve;
mod session;
pub mod skills;
mod taxonomy;
mod verify;

pub use agentic::{computer_use, world_model_claim, ToolResult};
pub use answer::{Answer, Layer};
pub use benchmark_db::{
    all_benchmarks, bench_by_id, benches_for_domain, superhuman_benches, BenchmarkEntry,
};
pub use card::{cannot_know, model_card, published_closed_score_is_ours, ModelCard, Tier};
pub use citations::{
    citations_from_chunks, citations_from_context, citations_from_hits, id_for_text,
    lookup as lookup_citations, CitationError,
};
pub use cite::{retrieve_and_cite, retrieve_and_cite_hybrid, RetrieveCiteError, RetrievedCitations};
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
pub use hybrid::{rank_hybrid, rank_lexical};
pub use meta::{aikd_v1_tagged, audit_report, knows_everything};
pub use model_registry::{model_by_id, models_for_domain, open_models, ModelEntry};
pub use packs::{
    gaia_certifies_usmle_or_bar, insurer_automation, professional_disclaimer, ScienceAnswer,
};
pub use quality::{
    check_contradiction, check_entity_hallucination, hallucination_risk_score,
    temporal_validation, tier_for_claim, ContradictionFlag, EntityFlag, KnowledgeClaim,
    QualityTier, SourceType, TemporalValidation,
};
pub use rank::{rank_persisted, RankedHit};
pub use registry::{system_tags, BenchRow};
pub use retrieve::{embed_query, GenerationContext, QueryHit, RetrievedChunk};
pub use session::{embodied_enabled, sandbox_breakout, ToolCube};
pub use skills::{
    all_skills, assess_skill, learning_path, search_skills, skill_by_id, skill_gap_analysis,
    skills_for_domain, skills_for_tool, Skill, SkillAssessment, SkillCategory, SkillGap,
    SkillLearningPath,
};
pub use taxonomy::{cannot_know_catalog, query_cannot_know, KnowledgeLayer, KnowledgeType, Status};
pub use verify::Executed;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AikdError {
    CannotKnow,
    NeedVerify,
    ClosedScoreClaim,
    MissingCitation,
}

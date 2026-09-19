//! UKD first cuts (#76–#92). Not Neo4j, live SPARQL, or a v1.0 tag.

mod contribute;
mod graph;
mod i18n;
mod ingest;
mod intelligence;
mod learn;
mod literature;
mod mark;
mod modes;
mod path;
mod realms;
mod sovereignty;
mod store;
mod teach;
mod tek;

pub use contribute::{GraphView, NodeDraft};
pub use graph::{federated_wikidata, get_node, list_realms, KnowledgeNode, REALMS};
pub use i18n::{
    fallback_chain, public_dump, release_notes, sign_language_links, switch_ui, UN_LANGS,
};
pub use ingest::Ingested;
pub use intelligence::{Claim, ReviewQueue, Triple};
pub use learn::{labels_for, plan, Labels, PlannedPath};
pub use literature::{present, KnowledgeState, LicensedNode};
pub use mark::{qa_script, ExportMark};
pub use modes::{earth_twin_cites, teach_offline, tek_export, ukd_v1_tagged, GaianMode};
pub use path::{path_to_quantum_computing, LearningPath};
pub use realms::{cross_domain_candidates, indigenous_parallel, subjects};
pub use sovereignty::{Agreement, CollectionState, TekGraph};
pub use store::{Edge, GraphApi};
pub use teach::{climate_boundary_links, offline_pack, sm2, teach_me};
pub use tek::{publish_tek, GaianKnowledge, KnowledgeLevel, TekStore};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UkdError {
    UnknownNode,
    TekSealed,
    Uncited,
    NoAgreement,
    ChildBlocked,
}

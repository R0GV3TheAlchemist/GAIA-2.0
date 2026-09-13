//! UKD first cuts (#76–#85). Not Neo4j, live SPARQL, or a v1.0 tag.

mod graph;
mod ingest;
mod intelligence;
mod mark;
mod modes;
mod path;
mod store;
mod tek;

pub use graph::{federated_wikidata, get_node, list_realms, KnowledgeNode, REALMS};
pub use ingest::Ingested;
pub use intelligence::{Claim, ReviewQueue, Triple};
pub use mark::{qa_script, ExportMark};
pub use modes::{earth_twin_cites, teach_offline, tek_export, ukd_v1_tagged, GaianMode};
pub use path::{path_to_quantum_computing, LearningPath};
pub use store::{Edge, GraphApi};
pub use tek::{publish_tek, GaianKnowledge, KnowledgeLevel, TekStore};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UkdError {
    UnknownNode,
    TekSealed,
    Uncited,
    NoAgreement,
    ChildBlocked,
}

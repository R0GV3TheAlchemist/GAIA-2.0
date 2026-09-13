//! UKD first cuts (#77–#80) plus a GAIAN export-mark fixture (#76).
//! Not Neo4j, SPARQL live, c2pa-rs, or a v1.0 tag.

mod graph;
mod mark;
mod path;
mod tek;

pub use graph::{federated_wikidata, get_node, list_realms, KnowledgeNode, REALMS};
pub use mark::{qa_script, ExportMark};
pub use path::{path_to_quantum_computing, LearningPath};
pub use tek::{publish_tek, GaianKnowledge, KnowledgeLevel, TekStore};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UkdError {
    UnknownNode,
    TekSealed,
    Uncited,
    NoAgreement,
    ChildBlocked,
}

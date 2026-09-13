//! HSPD first cuts (#132–#140). Ethics before catalog. Not a clinic.

mod catalog;
mod charter;
mod node;
mod path;
mod profile;

pub use catalog::{nodes_for, REALMS};
pub use charter::{ban_list, principles};
pub use node::{parse_node, RiskClass, SuperNode};
pub use path::{flow_path, freediving_note, pharma_path};
pub use profile::HspdProfile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HspdError {
    DoseForbidden,
    DiyPathForbidden,
    ChildGenetic,
    ResearchOnly,
    NotPractice,
    PharmacyListing,
}

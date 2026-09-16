//! Formal geometry and First-Law symbol contracts.
//! Art is allowed. Operational claims require a complete contract.
//! Not consciousness. Not TEK extraction. Not an aggregate flourishing score.

mod compass;
mod diamond;
mod primitive;
mod registry;

pub use compass::{Compass, Cardinal};
pub use diamond::{Alignment, Gate};
pub use primitive::{Circle, Point, Vesica};
pub use registry::{lookup, operational, SymbolContract, SymbolKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeoError {
    IncompleteContract,
    AggregateScoreForbidden,
    TekSealed,
    CategoryCollapse,
    CommandClaim,
    MissingEvidence,
    MissingSteward,
}

pub fn geometry_v1_tagged() -> bool {
    false
}

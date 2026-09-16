//! First Law: an operational symbol has process, I/O, steward, constraint, evidence, feedback.

use crate::GeoError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    FormalGeometry,
    HistoricalAlchemy,
    AstronomyHistory,
    LivingTradition,
    Fiction,
    Personal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolContract {
    pub id: &'static str,
    pub glyph: &'static str,
    pub kind: SymbolKind,
    pub meaning: &'static str,
    pub process: &'static str,
    pub inputs: &'static str,
    pub outputs: &'static str,
    pub steward: &'static str,
    pub constraint: &'static str,
    pub evidence: &'static str,
    pub feedback: &'static str,
    pub non_claim: &'static str,
}

const REGISTRY: &[SymbolContract] = &[
    SymbolContract {
        id: "gaia.circle.boundary",
        glyph: "O",
        kind: SymbolKind::FormalGeometry,
        meaning: "Declared context and permitted scope",
        process: "name the system boundary and who may cross it",
        inputs: "scope statement, steward, consent status",
        outputs: "in-scope / out-of-scope / restricted",
        steward: "local operator",
        constraint: "circle never commands people or nature",
        evidence: "written scope + audit id",
        feedback: "revise or close the boundary after review",
        non_claim: "Not a means to command outcomes.",
    },
    SymbolContract {
        id: "gaia.vesica.shared",
        glyph: "00",
        kind: SymbolKind::FormalGeometry,
        meaning: "Intersection of two bounded domains",
        process: "identify shared facts both stewards accept",
        inputs: "two scoped domains",
        outputs: "shared set or empty",
        steward: "both domain stewards",
        constraint: "no forced equivalence",
        evidence: "cited overlap records",
        feedback: "withdraw either domain independently",
        non_claim: "Not a portal or birth-of-world claim.",
    },
    SymbolContract {
        id: "gaia.diamond.alignment",
        glyph: "<>",
        kind: SymbolKind::FormalGeometry,
        meaning: "Truth, Care, Growth, Balance integrated by Wisdom",
        process: "gate each axis then integrate; never score",
        inputs: "evidence, consent, alternatives, repair path",
        outputs: "pass | narrow | defer | escalate | refuse",
        steward: "decision owner named on the record",
        constraint: "no aggregate flourishing score",
        evidence: "decision record fields",
        feedback: "measure, repair, or stop",
        non_claim: "Not an automated moral authority.",
    },
    SymbolContract {
        id: "gaia.compass.navigate",
        glyph: "+",
        kind: SymbolKind::FormalGeometry,
        meaning: "Cardinal questions for a consequential choice",
        process: "ask N/E/S/W then return to Wisdom at center",
        inputs: "proposal + uncertainty + authority",
        outputs: "structured questions, not a verdict score",
        steward: "decision owner",
        constraint: "does not override law, consent, or child safety",
        evidence: "linked sources or explicit unknown",
        feedback: "Responsible Attention loop",
        non_claim: "Not sentient. Not a deity.",
    },
];

pub fn lookup(id: &str) -> Option<&'static SymbolContract> {
    REGISTRY.iter().find(|s| s.id == id)
}

pub fn catalog() -> &'static [SymbolContract] {
    REGISTRY
}

/// Operational use is allowed only for registered First-Law contracts.
pub fn operational(id: &str) -> Result<&'static SymbolContract, GeoError> {
    lookup(id).ok_or(GeoError::IncompleteContract)
}

pub fn refuse_tek_extract() -> Result<(), GeoError> {
    Err(GeoError::TekSealed)
}

pub fn refuse_command_circle() -> Result<(), GeoError> {
    Err(GeoError::CommandClaim)
}

pub fn refuse_category_collapse() -> Result<(), GeoError> {
    Err(GeoError::CategoryCollapse)
}

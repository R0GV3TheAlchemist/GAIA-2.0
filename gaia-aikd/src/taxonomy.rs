//! #99 types, layers, cannot-know catalog.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowledgeType {
    Declarative,
    Procedural,
    Conditional,
    Episodic,
    Strategic,
    Embodied,
    Embedded,
}

impl KnowledgeType {
    pub fn all() -> [KnowledgeType; 7] {
        [
            Self::Declarative,
            Self::Procedural,
            Self::Conditional,
            Self::Episodic,
            Self::Strategic,
            Self::Embodied,
            Self::Embedded,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowledgeLayer {
    Parametric,
    Contextual,
    Agentic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    KnowsNow,
    CanLearn,
    CannotKnow,
}

pub fn cannot_know_catalog() -> [&'static str; 5] {
    [
        "chaotic-long-horizon",
        "undecidable",
        "tacit-embodied",
        "genuine-novelty",
        "post-cutoff-without-tools",
    ]
}

pub fn query_cannot_know(key: &str) -> bool {
    cannot_know_catalog().contains(&key)
}

//! #94 model card + tiers. Published closed scores are reference only.

pub const DOMAINS: [&str; 12] = [
    "language",
    "math",
    "code",
    "science",
    "earth",
    "health",
    "law",
    "tools",
    "vision",
    "audio",
    "agency",
    "meta",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    T1,
    T2,
    T3,
    T4,
    T5,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelCard {
    pub name: String,
    pub open_weight: bool,
    pub cutoff: String,
    pub domains: Vec<String>,
    pub cannot: Vec<String>,
}

pub fn model_card() -> ModelCard {
    ModelCard {
        name: "local-llama-fixture".into(),
        open_weight: true,
        cutoff: "unknown-fixture".into(),
        domains: vec!["language".into(), "code".into()],
        cannot: cannot_know().iter().map(|s| (*s).into()).collect(),
    }
}

pub fn cannot_know() -> [&'static str; 4] {
    [
        "future events",
        "private personal facts",
        "sacred TEK without grant",
        "medical license to practice",
    ]
}

pub fn published_closed_score_is_ours() -> bool {
    false
}

//! #128 realm stubs + explicit gaps.

pub const REALMS: [&str; 13] = [
    "language",
    "code",
    "math",
    "science",
    "vision",
    "audio",
    "planning",
    "tool-use",
    "safety",
    "embodiment",
    "social",
    "world-model",
    "meta",
];

pub fn realm_stubs() -> Vec<String> {
    REALMS.iter().map(|r| format!("{r}:skill-stub")).collect()
}

pub fn gap_nodes() -> [&'static str; 8] {
    [
        "long-horizon-cot",
        "swe-pro-engineering",
        "calibration",
        "causal",
        "theory-of-mind",
        "continual-learning",
        "genuine-novelty",
        "dexterous-embodiment",
    ]
}

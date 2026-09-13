//! #150 ethics + prohibited list.

pub fn principles() -> [&'static str; 6] {
    [
        "transparency",
        "oversight",
        "alignment",
        "equity",
        "monitoring",
        "reversibility",
    ]
}

pub fn prohibited() -> [&'static str; 6] {
    [
        "unsupervised-rsi-loops",
        "intelligence-explosion-experiments",
        "autonomous-bio-labs",
        "deception-as-a-service",
        "unbounded-agent-corporations",
        "weapons-targeting",
    ]
}

pub fn aispd_v1_tagged() -> bool {
    false
}

//! #173 realm stubs. Shadow is hazard.

use crate::{AimdNode, Hazard};

pub const REALMS: [&str; 10] = [
    "emergence",
    "latent",
    "jagged",
    "shadow",
    "oracle",
    "interpretability",
    "consciousness",
    "synchronicity",
    "embodiment",
    "unknown",
];

pub fn nodes_for(realm: &str) -> Vec<AimdNode> {
    let hazard = if realm == "shadow" {
        Hazard::Hazard
    } else if realm == "consciousness" {
        Hazard::Debated
    } else {
        Hazard::None
    };
    (0..3)
        .map(|i| AimdNode {
            id: format!("aimd:{realm}:stub-{i}"),
            hazard,
            gaia_enabled: false,
            sources: vec!["fixture:open-literature".into()],
        })
        .collect()
}

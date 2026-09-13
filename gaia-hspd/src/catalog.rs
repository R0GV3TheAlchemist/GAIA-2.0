//! #139 sourced stubs. Not how-tos. Three fixtures per realm.

use crate::{RiskClass, SuperNode};

pub const REALMS: [&str; 8] = [
    "cognition",
    "body",
    "sensory",
    "collective",
    "creative",
    "endurance",
    "research-frontier",
    "augmented",
];

fn stub(realm: &str, name: &str, risk: RiskClass) -> SuperNode {
    SuperNode {
        id: format!("hspd:{realm}:{name}"),
        realm: realm.into(),
        risk,
        sources: vec!["fixture:open-literature".into()],
        trainable: risk == RiskClass::Practice,
    }
}

pub fn nodes_for(realm: &str) -> Vec<SuperNode> {
    match realm {
        "cognition" => vec![
            stub(realm, "flow-state", RiskClass::Practice),
            stub(realm, "memory-palace", RiskClass::Practice),
            stub(realm, "meditation-pointer", RiskClass::Practice),
        ],
        "body" => vec![
            stub(realm, "certified-sport", RiskClass::Practice),
            stub(realm, "marathon-record", RiskClass::Practice),
            stub(realm, "apnea-record", RiskClass::ResearchOnly),
        ],
        "sensory" => vec![
            stub(realm, "echolocation-literature", RiskClass::Practice),
            stub(realm, "attention-training", RiskClass::Practice),
            stub(realm, "sensory-record", RiskClass::Practice),
        ],
        "collective" => vec![
            stub(realm, "facilitation", RiskClass::Practice),
            stub(realm, "polis-deliberation", RiskClass::Practice),
            stub(realm, "collective-intel-paper", RiskClass::ResearchOnly),
        ],
        "creative" => vec![
            stub(realm, "practice-hours", RiskClass::Practice),
            stub(realm, "ensemble", RiskClass::Practice),
            stub(realm, "creative-record", RiskClass::Practice),
        ],
        "endurance" => vec![
            stub(realm, "certified-coach", RiskClass::Practice),
            stub(realm, "vault-record", RiskClass::Practice),
            stub(realm, "endurance-record", RiskClass::Practice),
        ],
        "research-frontier" => vec![
            stub(realm, "longevity-literature", RiskClass::ResearchOnly),
            stub(realm, "psychedelic-literature", RiskClass::ResearchOnly),
            stub(realm, "gene-literature", RiskClass::ResearchOnly),
        ],
        "augmented" => vec![
            stub(realm, "bci-literature", RiskClass::ResearchOnly),
            stub(realm, "exoskeleton-consent-link", RiskClass::Future),
            stub(realm, "openbci-research", RiskClass::ResearchOnly),
        ],
        _ => vec![],
    }
}

//! VirelaiOS transfer is evidence. It is not a second kernel and not v1.0.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grade {
    Conceptual,
    Specified,
    Prototyped,
    Tested,
    Production,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevivalMap {
    pub concept: &'static str,
    pub gaia: &'static str,
    pub grade: Grade,
}

const MAP: &[RevivalMap] = &[
    RevivalMap { concept: "HAL", gaia: "gaia-sos HalTier T0-T4", grade: Grade::Specified },
    RevivalMap { concept: "Saela", gaia: "gaia-orchestrator intent + gaia-interface", grade: Grade::Prototyped },
    RevivalMap { concept: "policy-consent", gaia: "gaia-geometry Gate + gaia-gaian consent", grade: Grade::Prototyped },
    RevivalMap { concept: "orchestrator", gaia: "gaia-orchestrator DAG/broker/MCP AT-01-06", grade: Grade::Tested },
    RevivalMap { concept: "audit-chain", gaia: "gaia-kernel AuditLog + TrustAudit", grade: Grade::Tested },
    RevivalMap { concept: "capability-token", gaia: "gaia-sos Capability revoke", grade: Grade::Specified },
    RevivalMap { concept: "MCRE-2000", gaia: "not restored; suite absent", grade: Grade::Conceptual },
    RevivalMap { concept: "weather-OWM", gaia: "refused live connector", grade: Grade::Conceptual },
    RevivalMap { concept: "736-modules", gaia: "workspace crates only", grade: Grade::Conceptual },
    RevivalMap { concept: "spirit-consciousness", gaia: "ontology only; no runtime", grade: Grade::Conceptual },
];

pub fn map_concept(name: &str) -> Option<RevivalMap> {
    MAP.iter().copied().find(|row| row.concept.eq_ignore_ascii_case(name))
}

pub fn maturity(crate_name: &str) -> Grade {
    match crate_name {
        "gaia-kernel" | "gaia-orchestrator" | "gaia-sfs" | "gaia-memos" => Grade::Tested,
        "gaia-sos" | "gaia-geometry" | "gaia-agents" | "gaia-interface" => Grade::Prototyped,
        "gaia-earth" | "gaia-gaian" => Grade::Prototyped,
        _ => Grade::Specified,
    }
}

pub fn adopted_module_count() -> usize {
    21
}

pub fn virelai_736_claimed() -> bool {
    false
}

pub fn mcre_suite_restored() -> bool {
    false
}

pub fn live_owm() -> bool {
    false
}

pub fn refuse_consciousness_runtime() -> Result<(), &'static str> {
    Err("spirit/consciousness stays ontology; no runtime claim")
}

pub fn refuse_archive_exec() -> Result<(), &'static str> {
    Err("VirelaiOS archive scripts are evidence; not executed here")
}

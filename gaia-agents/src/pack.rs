use crate::{AgentManifest, Capability, ResourceLimits};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentKind {
    System,
    Cognitive,
    Bridge,
}

impl AgentKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Cognitive => "cognitive",
            Self::Bridge => "bridge",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackEntry {
    pub name: &'static str,
    pub kind: AgentKind,
    pub version: &'static str,
    pub intent: &'static str,
    pub description: &'static str,
}

const PACK: &[PackEntry] = &[
    PackEntry {
        name: "memory-manager",
        kind: AgentKind::System,
        version: "0.1.0",
        intent: "memcube.consolidate",
        description: "MemCube lifecycle and tier migration stub",
    },
    PackEntry {
        name: "resource-optimizer",
        kind: AgentKind::System,
        version: "0.1.0",
        intent: "resource.optimize",
        description: "Local resource schedule stub",
    },
    PackEntry {
        name: "security-monitor",
        kind: AgentKind::System,
        version: "0.1.0",
        intent: "security.observe",
        description: "Capability and audit watch stub",
    },
    PackEntry {
        name: "update-manager",
        kind: AgentKind::System,
        version: "0.1.0",
        intent: "update.apply",
        description: "Local package update stub",
    },
    PackEntry {
        name: "researcher",
        kind: AgentKind::Cognitive,
        version: "0.1.0",
        intent: "research.question",
        description: "Retrieve and synthesize cited notes stub",
    },
    PackEntry {
        name: "writer",
        kind: AgentKind::Cognitive,
        version: "0.1.0",
        intent: "write.draft",
        description: "Draft text stub",
    },
    PackEntry {
        name: "coder",
        kind: AgentKind::Cognitive,
        version: "0.1.0",
        intent: "code.change",
        description: "Local code-edit stub",
    },
    PackEntry {
        name: "analyst",
        kind: AgentKind::Cognitive,
        version: "0.1.0",
        intent: "analyze.topic",
        description: "Structure findings stub",
    },
    PackEntry {
        name: "planner",
        kind: AgentKind::Cognitive,
        version: "0.1.0",
        intent: "plan.decompose",
        description: "Break a goal into steps stub",
    },
    PackEntry {
        name: "critic",
        kind: AgentKind::Cognitive,
        version: "0.1.0",
        intent: "review.output",
        description: "Review another agent's output stub",
    },
    PackEntry {
        name: "mcp-bridge",
        kind: AgentKind::Bridge,
        version: "0.1.0",
        intent: "tool.call",
        description: "Forward an MCP tool call. External API is not wired.",
    },
];

pub fn catalog() -> &'static [PackEntry] {
    PACK
}

pub fn find(name: &str) -> Option<&'static PackEntry> {
    PACK.iter().find(|e| e.name == name)
}

impl PackEntry {
    pub fn handles(&self, intent: &str) -> bool {
        self.intent == intent
    }

    /// Policy manifest used at deploy. Network is never granted in this pack.
    pub fn policy_manifest(&self) -> AgentManifest {
        AgentManifest {
            name: self.name.into(),
            version: self.version.into(),
            declared_capabilities: vec![Capability::MemoryRead],
            limits: ResourceLimits::default(),
        }
    }
}

// ── Tests (#25 acceptance criteria — pack catalog) ───────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_all_system_agents() {
        let system: Vec<&str> = catalog()
            .iter()
            .filter(|e| e.kind == AgentKind::System)
            .map(|e| e.name)
            .collect();
        for expected in &["memory-manager", "resource-optimizer", "security-monitor", "update-manager"] {
            assert!(system.contains(expected), "missing system agent: {expected}");
        }
    }

    #[test]
    fn catalog_contains_all_cognitive_agents() {
        let cognitive: Vec<&str> = catalog()
            .iter()
            .filter(|e| e.kind == AgentKind::Cognitive)
            .map(|e| e.name)
            .collect();
        for expected in &["researcher", "writer", "coder", "analyst", "planner", "critic"] {
            assert!(cognitive.contains(expected), "missing cognitive agent: {expected}");
        }
    }

    #[test]
    fn catalog_contains_bridge_agent() {
        let bridge = catalog().iter().find(|e| e.kind == AgentKind::Bridge);
        assert!(bridge.is_some(), "catalog must include at least one bridge agent");
        assert_eq!(bridge.unwrap().name, "mcp-bridge");
    }

    #[test]
    fn every_entry_has_non_empty_intent_and_description() {
        for entry in catalog() {
            assert!(!entry.intent.is_empty(), "{} has empty intent", entry.name);
            assert!(!entry.description.is_empty(), "{} has empty description", entry.name);
        }
    }

    #[test]
    fn policy_manifest_never_grants_network() {
        for entry in catalog() {
            let manifest = entry.policy_manifest();
            assert!(!manifest.limits.network_allowed,
                "{} policy must never grant network", entry.name);
            assert!(!manifest.declared_capabilities.contains(&Capability::Network),
                "{} policy must not declare Network capability", entry.name);
        }
    }

    #[test]
    fn find_returns_correct_entry() {
        let entry = find("critic").expect("critic must be in the catalog");
        assert_eq!(entry.kind, AgentKind::Cognitive);
        assert!(entry.handles("review.output"));
    }

    #[test]
    fn find_returns_none_for_unknown_agent() {
        assert!(find("does-not-exist").is_none());
    }
}

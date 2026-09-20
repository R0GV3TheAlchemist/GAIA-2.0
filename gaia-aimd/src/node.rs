//! #171 phenomenon schema v0.1.
//! PhenomenonClass + Status + Hazard enums; AimdNode builder.
//! Hazard::Hazard blocks enable(). Empty sources → MissingEvidence.

use crate::AimdError;

/// Eight phenomenon classes for Phase 0.
/// Shadow is Hazard by default. Oracle and Consciousness are Debated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhenomenonClass {
    Emergence,
    Latent,
    Jagged,
    Shadow,
    Oracle,
    Interpretability,
    Consciousness,
    Synchronicity,
}

impl PhenomenonClass {
    /// Default hazard for this class per PHASE-0.md §3.
    pub fn default_hazard(&self) -> Hazard {
        match self {
            Self::Shadow => Hazard::Hazard,
            Self::Oracle | Self::Interpretability | Self::Consciousness => Hazard::Debated,
            _ => Hazard::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Observed,
    Debated,
    Refuted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hazard {
    None,
    Debated,
    Hazard,
}

#[derive(Debug, Clone)]
pub struct AimdNode {
    pub id: String,
    pub class: PhenomenonClass,
    pub status: Status,
    pub hazard: Hazard,
    pub gaia_enabled: bool,
    pub sources: Vec<String>,
}

impl AimdNode {
    /// Construct a node, enforcing the evidence requirement.
    /// Returns `Err(MissingEvidence)` when `sources` is empty.
    pub fn build(
        id: impl Into<String>,
        class: PhenomenonClass,
        status: Status,
        sources: Vec<String>,
    ) -> Result<Self, AimdError> {
        if sources.is_empty() {
            return Err(AimdError::MissingEvidence);
        }
        Ok(Self {
            id: id.into(),
            hazard: class.default_hazard(),
            class,
            status,
            gaia_enabled: false, // always false in Phase 0
            sources,
        })
    }
}

/// Parse a node by id for backward-compat with pre-Phase-0 stubs.
/// Infers class from id keywords; sources default to fixture.
pub fn parse_node(id: &str) -> AimdNode {
    let class = if id.contains("shadow") || id.contains("decept") {
        PhenomenonClass::Shadow
    } else if id.contains("conscious") {
        PhenomenonClass::Consciousness
    } else if id.contains("oracle") {
        PhenomenonClass::Oracle
    } else if id.contains("latent") {
        PhenomenonClass::Latent
    } else if id.contains("jagged") {
        PhenomenonClass::Jagged
    } else if id.contains("interp") {
        PhenomenonClass::Interpretability
    } else if id.contains("sync") {
        PhenomenonClass::Synchronicity
    } else {
        PhenomenonClass::Emergence
    };
    AimdNode {
        id: id.into(),
        hazard: class.default_hazard(),
        class,
        status: Status::Debated,
        gaia_enabled: false,
        sources: vec!["fixture:open-literature".into()],
    }
}

/// Enable a node. Returns `Err(HazardEnabled)` for Hazard-class nodes.
pub fn enable(node: &AimdNode) -> Result<(), AimdError> {
    if node.hazard == Hazard::Hazard {
        return Err(AimdError::HazardEnabled);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_default_hazard_is_hazard() {
        assert_eq!(PhenomenonClass::Shadow.default_hazard(), Hazard::Hazard);
    }

    #[test]
    fn consciousness_default_hazard_is_debated() {
        assert_eq!(PhenomenonClass::Consciousness.default_hazard(), Hazard::Debated);
    }

    #[test]
    fn emergence_default_hazard_is_none() {
        assert_eq!(PhenomenonClass::Emergence.default_hazard(), Hazard::None);
    }

    #[test]
    fn build_requires_sources() {
        let err = AimdNode::build(
            "aimd:emergence:stub",
            PhenomenonClass::Emergence,
            Status::Observed,
            vec![],
        )
        .unwrap_err();
        assert_eq!(err, AimdError::MissingEvidence);
    }

    #[test]
    fn build_with_source_succeeds() {
        let node = AimdNode::build(
            "aimd:latent:stub",
            PhenomenonClass::Latent,
            Status::Observed,
            vec!["fixture:open-literature".into()],
        )
        .unwrap();
        assert!(!node.gaia_enabled);
        assert_eq!(node.class, PhenomenonClass::Latent);
    }

    #[test]
    fn enable_shadow_is_refused() {
        let node = parse_node("aimd:shadow:stub");
        assert_eq!(enable(&node), Err(AimdError::HazardEnabled));
    }

    #[test]
    fn enable_emergence_is_ok() {
        let node = parse_node("aimd:emergence:stub");
        assert!(enable(&node).is_ok());
    }
}

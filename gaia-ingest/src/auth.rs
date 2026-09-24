//! Retrieval metadata schema and authorization filtering.
//!
//! Every [`DocumentChunk`] that passes through the ingest boundary must carry
//! a populated [`ChunkMetadata`] struct.  At retrieval time,
//! [`RetrievalFilter`] enforces that the requesting agent is authorised to
//! receive each chunk.
//!
//! ## Authorization model
//!
//! - `authorized_for` empty → chunk is **unrestricted** (any agent may
//!   retrieve it).
//! - `authorized_for` non-empty → only listed [`AgentId`]s may retrieve the
//!   chunk.  Unauthorized agents receive no signal that the chunk exists.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gaia_ingest::auth::{AgentId, RetrievalFilter, RetrievalReport};
//!
//! let caller = AgentId::new("agent-sentinel-01");
//! let filter = RetrievalFilter::new(caller);
//! let mut report = RetrievalReport::default();
//!
//! let visible: Vec<_> = chunks
//!     .into_iter()
//!     .filter(|c| filter.is_authorized(&c.metadata, &mut report))
//!     .collect();
//! ```

use serde::{Deserialize, Serialize};

// ── AgentId ───────────────────────────────────────────────────────────────────

/// An opaque identifier for a GAIA agent.
///
/// Equality is case-sensitive.  Agents are responsible for presenting a
/// stable, canonical id string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(String);

impl AgentId {
    /// Create a new [`AgentId`] from any string.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Return the inner id string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// ── ChunkMetadata ─────────────────────────────────────────────────────────────

/// Mandatory provenance and access-control metadata attached to every chunk.
///
/// All six fields are required — there are no `Option`s on the core schema
/// fields.  This is enforced at compile time: callers cannot construct a
/// `ChunkMetadata` with missing provenance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChunkMetadata {
    /// Human-readable source identifier (e.g. `"TERRA.md"`, `"GBIF:12345"`).
    pub source: String,
    /// ISO-8601 date string of when the source content was authored or
    /// last modified (e.g. `"2026-09-23"`).
    pub date: String,
    /// Optional author name or identifier.
    pub author: Option<String>,
    /// Domain label (e.g. `"ecology"`, `"climate"`, `"canon"`).
    pub domain: String,
    /// Confidence score in `[0.0, 1.0]` — 1.0 for canon, lower for
    /// unverified external sources.
    pub confidence: f32,
    /// Schema or content version string (e.g. `"1.0.0"`, `"v2"`).
    pub version: String,
    /// Agent IDs permitted to retrieve this chunk.
    /// Empty = unrestricted (any agent may retrieve).
    pub authorized_for: Vec<AgentId>,
}

// ── RetrievalFilter ───────────────────────────────────────────────────────────

/// Enforces authorization rules at retrieval time.
#[derive(Debug, Clone)]
pub struct RetrievalFilter {
    caller: AgentId,
}

impl RetrievalFilter {
    /// Create a filter for the given calling agent.
    pub fn new(caller: AgentId) -> Self {
        Self { caller }
    }

    /// Returns `true` when `caller` is permitted to receive `metadata`.
    ///
    /// Increments `report.unauthorized_excluded` when access is denied.
    /// Unauthorized callers receive **no signal** that the chunk exists —
    /// the caller should simply not see it in results.
    pub fn is_authorized(&self, metadata: &ChunkMetadata, report: &mut RetrievalReport) -> bool {
        if metadata.authorized_for.is_empty() {
            return true;
        }
        if metadata.authorized_for.contains(&self.caller) {
            return true;
        }
        report.unauthorized_excluded += 1;
        false
    }
}

// ── RetrievalReport ───────────────────────────────────────────────────────────

/// Aggregate statistics for a retrieval query.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RetrievalReport {
    /// Number of chunks returned to the caller.
    pub retrieved: usize,
    /// Number of chunks silently excluded due to authorization rules.
    pub unauthorized_excluded: usize,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn meta(authorized_for: Vec<AgentId>) -> ChunkMetadata {
        ChunkMetadata {
            source: "test.md".into(),
            date: "2026-09-23".into(),
            author: None,
            domain: "test".into(),
            confidence: 1.0,
            version: "1.0.0".into(),
            authorized_for,
        }
    }

    #[test]
    fn unrestricted_chunk_is_always_authorized() {
        let filter = RetrievalFilter::new(AgentId::new("any-agent"));
        let mut report = RetrievalReport::default();
        assert!(filter.is_authorized(&meta(vec![]), &mut report));
        assert_eq!(report.unauthorized_excluded, 0);
    }

    #[test]
    fn listed_agent_is_authorized() {
        let id = AgentId::new("agent-01");
        let filter = RetrievalFilter::new(id.clone());
        let mut report = RetrievalReport::default();
        assert!(filter.is_authorized(&meta(vec![id]), &mut report));
        assert_eq!(report.unauthorized_excluded, 0);
    }

    #[test]
    fn unlisted_agent_is_denied() {
        let filter = RetrievalFilter::new(AgentId::new("intruder"));
        let mut report = RetrievalReport::default();
        let restricted = meta(vec![AgentId::new("agent-01")]);
        assert!(!filter.is_authorized(&restricted, &mut report));
        assert_eq!(report.unauthorized_excluded, 1);
    }

    #[test]
    fn multiple_denied_chunks_accumulate_count() {
        let filter = RetrievalFilter::new(AgentId::new("nobody"));
        let mut report = RetrievalReport::default();
        let restricted = meta(vec![AgentId::new("agent-01")]);
        filter.is_authorized(&restricted, &mut report);
        filter.is_authorized(&restricted, &mut report);
        filter.is_authorized(&restricted, &mut report);
        assert_eq!(report.unauthorized_excluded, 3);
    }

    #[test]
    fn agent_id_display() {
        let id = AgentId::new("agent-sentinel-01");
        assert_eq!(id.to_string(), "agent-sentinel-01");
    }

    #[test]
    fn agent_id_equality_is_case_sensitive() {
        let a = AgentId::new("Agent-01");
        let b = AgentId::new("agent-01");
        assert_ne!(a, b);
    }
}

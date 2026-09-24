//! Retrieval guard — authorization filter for `gaia-memos` query path.
//!
//! This module extends the existing retrieval guard with
//! [`RetrievalFilter`] from `gaia-ingest::auth`, ensuring that chunks
//! with a non-empty `authorized_for` list are silently excluded for
//! agents not on that list.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gaia_memos::retrieval_guard::{
//!     MemosQuery, AuthorizedMemosResult,
//! };
//! use gaia_ingest::auth::AgentId;
//!
//! let query = MemosQuery::new(AgentId::new("agent-alpha"), "what did we discuss?");
//! let result = query.execute(candidate_memos);
//! println!(
//!     "retrieved={} excluded={}",
//!     result.report.retrieved,
//!     result.report.unauthorized_excluded,
//! );
//! ```

use gaia_ingest::auth::{AgentId, ChunkMetadata, RetrievalFilter, RetrievalReport};

// ── MemoCandidiate ────────────────────────────────────────────────────────────

/// A memo fragment candidate for retrieval.
#[derive(Debug, Clone, PartialEq)]
pub struct MemoCandidate {
    /// The memo text content.
    pub text: String,
    /// Provenance and authorization metadata.
    pub metadata: ChunkMetadata,
}

impl MemoCandidate {
    pub fn new(text: impl Into<String>, metadata: ChunkMetadata) -> Self {
        Self { text: text.into(), metadata }
    }
}

// ── MemosQuery ────────────────────────────────────────────────────────────────

/// A memos retrieval query bound to a caller [`AgentId`].
///
/// Calling [`MemosQuery::execute`] filters the candidate list through
/// [`RetrievalFilter`] and returns only the chunks the caller is
/// authorized to see, along with a [`RetrievalReport`] of the counts.
#[derive(Debug, Clone)]
pub struct MemosQuery {
    pub caller: AgentId,
    pub query_text: String,
}

impl MemosQuery {
    pub fn new(caller: AgentId, query_text: impl Into<String>) -> Self {
        Self { caller, query_text: query_text.into() }
    }

    /// Filter `candidates` for the caller and return authorized memos
    /// alongside the [`RetrievalReport`].
    pub fn execute(&self, candidates: Vec<MemoCandidate>) -> AuthorizedMemosResult {
        let filter = RetrievalFilter::new();
        let mut report = RetrievalReport::default();
        let mut memos = Vec::with_capacity(candidates.len());

        for candidate in candidates {
            if !filter.is_authorized(&self.caller, &candidate.metadata) {
                report.unauthorized_excluded += 1;
                continue;
            }
            report.retrieved += 1;
            memos.push(candidate);
        }

        AuthorizedMemosResult { memos, report }
    }
}

// ── AuthorizedMemosResult ─────────────────────────────────────────────────────

/// The output of [`MemosQuery::execute`]: the filtered memo list and the
/// authorization report.
#[derive(Debug, Clone, PartialEq)]
pub struct AuthorizedMemosResult {
    /// Memos the caller is authorized to see, in retrieval order.
    pub memos: Vec<MemoCandidate>,
    /// Authorization counts: how many were retrieved vs. excluded.
    pub report: RetrievalReport,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use gaia_ingest::auth::ChunkMetadata;

    fn meta(authorized_for: Vec<&str>) -> ChunkMetadata {
        ChunkMetadata {
            source: "memos-test".into(),
            date: "2026-09-01".into(),
            author: "gaia".into(),
            domain: "memos".into(),
            confidence: 1.0,
            version: "1".into(),
            authorized_for: authorized_for
                .into_iter()
                .map(|s| AgentId::new(s))
                .collect(),
        }
    }

    fn query(agent: &str) -> MemosQuery {
        MemosQuery::new(AgentId::new(agent), "test query")
    }

    // ── #942 acceptance criteria ──────────────────────────────────────────

    #[test]
    fn unrestricted_memo_accessible_to_all() {
        let candidates = vec![
            MemoCandidate::new("open memo", meta(vec![])),
        ];
        let result = query("anyone").execute(candidates);
        assert_eq!(result.memos.len(), 1);
        assert_eq!(result.report.retrieved, 1);
        assert_eq!(result.report.unauthorized_excluded, 0);
    }

    #[test]
    fn authorized_agent_receives_restricted_memo() {
        let candidates = vec![
            MemoCandidate::new("secret memo", meta(vec!["agent-alpha"])),
        ];
        let result = query("agent-alpha").execute(candidates);
        assert_eq!(result.memos.len(), 1);
        assert_eq!(result.report.unauthorized_excluded, 0);
    }

    #[test]
    fn unauthorized_agent_excluded_silently() {
        let candidates = vec![
            MemoCandidate::new("secret memo", meta(vec!["agent-alpha"])),
        ];
        let result = query("agent-beta").execute(candidates);
        assert!(result.memos.is_empty());
        assert_eq!(result.report.unauthorized_excluded, 1);
        assert_eq!(result.report.retrieved, 0);
    }

    #[test]
    fn mixed_batch_filters_correctly() {
        let candidates = vec![
            MemoCandidate::new("open", meta(vec![])),
            MemoCandidate::new("mine", meta(vec!["agent-alpha"])),
            MemoCandidate::new("theirs", meta(vec!["agent-beta"])),
        ];
        let result = query("agent-alpha").execute(candidates);
        assert_eq!(result.memos.len(), 2);
        assert_eq!(result.report.retrieved, 2);
        assert_eq!(result.report.unauthorized_excluded, 1);
    }

    #[test]
    fn counter_accumulates_across_multiple_exclusions() {
        let candidates = vec![
            MemoCandidate::new("a", meta(vec!["agent-z"])),
            MemoCandidate::new("b", meta(vec!["agent-z"])),
            MemoCandidate::new("c", meta(vec!["agent-z"])),
            MemoCandidate::new("open", meta(vec![])),
        ];
        let result = query("agent-alpha").execute(candidates);
        assert_eq!(result.report.unauthorized_excluded, 3);
        assert_eq!(result.report.retrieved, 1);
    }
}

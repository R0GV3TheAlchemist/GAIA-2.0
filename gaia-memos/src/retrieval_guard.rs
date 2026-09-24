//! RAG retrieval guard — enforces the Order ↔ Chaos lexicon-plane boundary
//! and agent-level authorization filtering (Batch B, #942).
//!
//! ## Lexicon-plane guard (pre-existing)
//!
//! [`guard_chunk`] / [`guard_chunks`] enforce the Order ↔ Chaos boundary
//! on [`ChunkRow`] values retrieved from the persist layer.
//!
//! ## Agent authorization guard (Batch B)
//!
//! [`MemosQuery`] applies `gaia_ingest::auth::RetrievalFilter` to a slice of
//! [`MemoCandidate`]s, silently excluding chunks the caller is not authorized
//! to see and recording the count in [`AuthorizedMemosResult::report`].

use thiserror::Error;

use crate::persist::ChunkRow;
use gaia_ingest::auth::{AgentId, ChunkMetadata, RetrievalFilter, RetrievalReport};

// ── QueryPlane ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryPlane {
    Order,
    Chaos,
    Bridge,
}

impl QueryPlane {
    pub fn as_str(&self) -> &'static str {
        match self {
            QueryPlane::Order  => "Order",
            QueryPlane::Chaos  => "Chaos",
            QueryPlane::Bridge => "Bridge",
        }
    }
}

// ── LexiconPlaneMismatch ──────────────────────────────────────────────────────

#[derive(Debug, Error)]
#[error(
    "lexicon plane mismatch: query plane is `{query_plane}` \
     but chunk `{chunk_id}` has plane `{chunk_plane}`"
)]
pub struct LexiconPlaneMismatch {
    pub query_plane: String,
    pub chunk_id:    String,
    pub chunk_plane: String,
}

// ── guard_chunk / guard_chunks ────────────────────────────────────────────────

pub fn guard_chunk(
    query_plane: QueryPlane,
    chunk: &ChunkRow,
) -> Result<(), LexiconPlaneMismatch> {
    if query_plane == QueryPlane::Bridge {
        return Ok(());
    }
    if chunk.lexicon_plane == "Bridge" {
        return Ok(());
    }
    if chunk.lexicon_plane != query_plane.as_str() {
        return Err(LexiconPlaneMismatch {
            query_plane: query_plane.as_str().to_owned(),
            chunk_id:    chunk.id.clone(),
            chunk_plane: chunk.lexicon_plane.clone(),
        });
    }
    Ok(())
}

pub fn guard_chunks(
    query_plane: QueryPlane,
    chunks: Vec<ChunkRow>,
) -> Result<Vec<ChunkRow>, LexiconPlaneMismatch> {
    if query_plane == QueryPlane::Bridge {
        return Ok(chunks);
    }
    let mut out = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        guard_chunk(query_plane, &chunk)?;
        out.push(chunk);
    }
    Ok(out)
}

// ── Batch B: agent authorization guard (#942) ─────────────────────────────────

/// A single memo candidate with its provenance metadata.
#[derive(Debug, Clone)]
pub struct MemoCandidate {
    pub text:     String,
    pub metadata: ChunkMetadata,
}

/// A query issued by an agent against the memo store.
pub struct MemosQuery {
    pub caller: AgentId,
    pub query:  String,
}

/// The filtered result of a [`MemosQuery::execute`] call.
pub struct AuthorizedMemosResult {
    /// Memo candidates the caller is authorized to receive.
    pub memos:  Vec<MemoCandidate>,
    /// Authorization statistics for this query.
    pub report: RetrievalReport,
}

impl MemosQuery {
    pub fn new(caller: AgentId, query: impl Into<String>) -> Self {
        Self { caller, query: query.into() }
    }

    /// Filter `candidates` through [`RetrievalFilter`], silently excluding
    /// any chunk the caller is not authorized to receive.
    pub fn execute(&self, candidates: Vec<MemoCandidate>) -> AuthorizedMemosResult {
        let filter = RetrievalFilter::new(self.caller.clone());
        let mut report = RetrievalReport::default();
        let mut memos = Vec::with_capacity(candidates.len());
        for candidate in candidates {
            if filter.is_authorized(&candidate.metadata, &mut report) {
                memos.push(candidate);
            }
        }
        AuthorizedMemosResult { memos, report }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persist::ChunkRow;

    // ── lexicon-plane guard (pre-existing) ────────────────────────────────────

    fn make_chunk(id: &str, plane: &str) -> ChunkRow {
        ChunkRow {
            id:               id.into(),
            document_uri:     "gaia://test".into(),
            document_title:   "Test Doc".into(),
            text:             "test text".into(),
            char_count:       9,
            chunk_index:      0,
            total_chunks:     1,
            kind:             "Test".into(),
            domain:           "test".into(),
            language:         "en".into(),
            confidence:       "Canon".into(),
            access_tier:      "Public".into(),
            lexicon_plane:    plane.into(),
            lexicon_voice:    None,
            authored_at_unix: None,
            ttl_seconds:      None,
        }
    }

    #[test]
    fn bridge_query_passes_all_planes() {
        let chunks = vec![
            make_chunk("a", "Order"),
            make_chunk("b", "Chaos"),
            make_chunk("c", "Bridge"),
        ];
        assert_eq!(guard_chunks(QueryPlane::Bridge, chunks).unwrap().len(), 3);
    }

    #[test]
    fn order_query_rejects_chaos() {
        let chunks = vec![make_chunk("chaos-1", "Chaos")];
        let err = guard_chunks(QueryPlane::Order, chunks).unwrap_err();
        assert_eq!(err.query_plane, "Order");
        assert_eq!(err.chunk_plane, "Chaos");
    }

    #[test]
    fn chaos_query_rejects_order() {
        let chunks = vec![make_chunk("order-1", "Order")];
        let err = guard_chunks(QueryPlane::Chaos, chunks).unwrap_err();
        assert_eq!(err.query_plane, "Chaos");
        assert_eq!(err.chunk_plane, "Order");
    }

    // ── #942: agent authorization guard ──────────────────────────────────────

    fn memo(text: &str, authorized_for: Vec<&str>) -> MemoCandidate {
        MemoCandidate {
            text: text.into(),
            metadata: ChunkMetadata {
                source:         "test-source".into(),
                date:           "2026-09-23".into(),
                author:         None,
                domain:         "test".into(),
                confidence:     1.0,
                version:        "1".into(),
                authorized_for: authorized_for
                    .into_iter()
                    .map(|s| AgentId::new(s))
                    .collect(),
            },
        }
    }

    #[test]
    fn unrestricted_memo_visible_to_any_caller() {
        let q = MemosQuery::new(AgentId::new("agent-x"), "anything");
        let result = q.execute(vec![memo("open", vec![])]);
        assert_eq!(result.memos.len(), 1);
        assert_eq!(result.report.unauthorized_excluded, 0);
    }

    #[test]
    fn restricted_memo_excluded_for_wrong_caller() {
        let q = MemosQuery::new(AgentId::new("agent-x"), "query");
        let result = q.execute(vec![memo("secret", vec!["agent-y"])]);
        assert_eq!(result.memos.len(), 0);
        assert_eq!(result.report.unauthorized_excluded, 1);
    }

    #[test]
    fn authorized_caller_receives_restricted_memo() {
        let q = MemosQuery::new(AgentId::new("agent-a"), "query");
        let result = q.execute(vec![memo("private", vec!["agent-a"])]);
        assert_eq!(result.memos.len(), 1);
        assert_eq!(result.report.unauthorized_excluded, 0);
    }

    #[test]
    fn mixed_batch_filters_correctly() {
        let q = MemosQuery::new(AgentId::new("agent-a"), "query");
        let candidates = vec![
            memo("open",   vec![]),
            memo("secret", vec!["agent-b"]),
            memo("mine",   vec!["agent-a"]),
        ];
        let result = q.execute(candidates);
        assert_eq!(result.memos.len(), 2);
        assert_eq!(result.report.unauthorized_excluded, 1);
        assert_eq!(result.memos[0].text, "open");
        assert_eq!(result.memos[1].text, "mine");
    }

    #[test]
    fn report_counts_multiple_exclusions() {
        let q = MemosQuery::new(AgentId::new("nobody"), "query");
        let candidates = vec![
            memo("s1", vec!["agent-a"]),
            memo("s2", vec!["agent-a"]),
            memo("s3", vec!["agent-a"]),
        ];
        let result = q.execute(candidates);
        assert_eq!(result.memos.len(), 0);
        assert_eq!(result.report.unauthorized_excluded, 3);
    }
}

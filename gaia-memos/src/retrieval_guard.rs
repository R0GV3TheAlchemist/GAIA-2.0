//! RAG retrieval guard — enforces the Order ↔ Chaos lexicon-plane boundary.
//!
//! ## Purpose
//!
//! The GAIA knowledge graph partitions documents into three planes:
//!
//! | Plane | Meaning |
//! |---|---|
//! | `Order` | Canonical, structured, sacred/scholarly sources |
//! | `Chaos` | Colloquial, narrative, synthetic, or unverified sources |
//! | `Bridge` | Unclassified — classifier has not yet run |
//!
//! A RAG query issued from an Order context must never silently receive
//! Chaos chunks (and vice-versa). Mixing planes corrupts the epistemic
//! provenance of the response. Per **C30** (no silent failures), the guard
//! surfaces a hard error rather than quietly filtering or truncating.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gaia_memos::{guard_chunks, QueryPlane};
//!
//! let safe = guard_chunks(QueryPlane::Order, candidate_chunks)?;
//! // safe contains only Order + Bridge chunks
//! ```
//!
//! `Bridge` chunks always pass — they are awaiting classification and must
//! not block retrieval. `Bridge` as a `QueryPlane` is a pass-through:
//! no filtering, no error (used during cold-start before the classifier runs).

use thiserror::Error;

use crate::persist::ChunkRow;

// ── QueryPlane ────────────────────────────────────────────────────────────────

/// The lexicon-plane context of a RAG query.
///
/// Set by the caller based on the document context or user session plane.
/// Controls which [`ChunkRow`]s the guard will allow through.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryPlane {
    /// Query originates from an Order context.
    /// Allows: `Order` chunks + unclassified `Bridge` chunks.
    /// Rejects: `Chaos` chunks → [`LexiconPlaneMismatch`].
    Order,

    /// Query originates from a Chaos context.
    /// Allows: `Chaos` chunks + unclassified `Bridge` chunks.
    /// Rejects: `Order` chunks → [`LexiconPlaneMismatch`].
    Chaos,

    /// Pass-through — no plane filtering applied.
    /// Used during cold-start (classifier not yet run) or for
    /// explicitly cross-plane administrative queries.
    Bridge,
}

impl QueryPlane {
    /// Returns the string representation matching the `lexicon_plane` column.
    pub fn as_str(&self) -> &'static str {
        match self {
            QueryPlane::Order  => "Order",
            QueryPlane::Chaos  => "Chaos",
            QueryPlane::Bridge => "Bridge",
        }
    }
}

// ── LexiconPlaneMismatch ──────────────────────────────────────────────────────

/// Error emitted when a retrieved chunk's plane conflicts with the query plane.
///
/// Per **C30** (no silent failures), this error is never swallowed.
/// Callers must handle it explicitly — either by re-issuing the query with
/// a `Bridge` plane or by surfacing it to the caller as a provenance error.
#[derive(Debug, Error)]
#[error(
    "lexicon plane mismatch: query plane is `{query_plane}` \
     but chunk `{chunk_id}` has plane `{chunk_plane}`"
)]
pub struct LexiconPlaneMismatch {
    /// The plane context of the originating query.
    pub query_plane: String,
    /// The `id` of the offending [`ChunkRow`].
    pub chunk_id:    String,
    /// The `lexicon_plane` value of the offending chunk.
    pub chunk_plane: String,
}

// ── guard_chunk ───────────────────────────────────────────────────────────────

/// Check a single [`ChunkRow`] against the query plane.
///
/// Returns `Ok(())` if the chunk is compatible, or
/// `Err(LexiconPlaneMismatch)` if it would cross the plane boundary.
///
/// # Rules
///
/// | `query_plane` | chunk `lexicon_plane` | Result |
/// |---|---|---|
/// | `Bridge` | any | `Ok` — pass-through |
/// | `Order` | `"Order"` | `Ok` |
/// | `Order` | `"Bridge"` | `Ok` — unclassified, allow through |
/// | `Order` | `"Chaos"` | `Err` |
/// | `Chaos` | `"Chaos"` | `Ok` |
/// | `Chaos` | `"Bridge"` | `Ok` — unclassified, allow through |
/// | `Chaos` | `"Order"` | `Err` |
pub fn guard_chunk(
    query_plane: QueryPlane,
    chunk: &ChunkRow,
) -> Result<(), LexiconPlaneMismatch> {
    // Bridge queries are always pass-through.
    if query_plane == QueryPlane::Bridge {
        return Ok(());
    }

    // Unclassified Bridge chunks are always compatible.
    if chunk.lexicon_plane == "Bridge" {
        return Ok(());
    }

    // Plane must match exactly.
    if chunk.lexicon_plane != query_plane.as_str() {
        return Err(LexiconPlaneMismatch {
            query_plane: query_plane.as_str().to_owned(),
            chunk_id:    chunk.id.clone(),
            chunk_plane: chunk.lexicon_plane.clone(),
        });
    }

    Ok(())
}

// ── guard_chunks ──────────────────────────────────────────────────────────────

/// Filter a batch of [`ChunkRow`]s against the query plane.
///
/// Returns `Ok(Vec<ChunkRow>)` containing only plane-compatible rows.
/// Returns `Err(LexiconPlaneMismatch)` on the **first** incompatible chunk
/// found — the error is not swallowed or logged-and-continued (C30).
///
/// Callers that want best-effort filtering instead of a hard error should
/// call [`guard_chunk`] per row and collect results themselves.
pub fn guard_chunks(
    query_plane: QueryPlane,
    chunks: Vec<ChunkRow>,
) -> Result<Vec<ChunkRow>, LexiconPlaneMismatch> {
    // Bridge is a pass-through — return immediately without iterating.
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

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persist::ChunkRow;

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
        let result = guard_chunks(QueryPlane::Bridge, chunks).unwrap();
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn order_query_passes_order_and_bridge() {
        let chunks = vec![
            make_chunk("a", "Order"),
            make_chunk("b", "Bridge"),
        ];
        let result = guard_chunks(QueryPlane::Order, chunks).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn order_query_rejects_chaos() {
        let chunks = vec![make_chunk("chaos-1", "Chaos")];
        let err = guard_chunks(QueryPlane::Order, chunks).unwrap_err();
        assert_eq!(err.query_plane, "Order");
        assert_eq!(err.chunk_plane, "Chaos");
        assert_eq!(err.chunk_id, "chaos-1");
    }

    #[test]
    fn chaos_query_passes_chaos_and_bridge() {
        let chunks = vec![
            make_chunk("a", "Chaos"),
            make_chunk("b", "Bridge"),
        ];
        let result = guard_chunks(QueryPlane::Chaos, chunks).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn chaos_query_rejects_order() {
        let chunks = vec![make_chunk("order-1", "Order")];
        let err = guard_chunks(QueryPlane::Chaos, chunks).unwrap_err();
        assert_eq!(err.query_plane, "Chaos");
        assert_eq!(err.chunk_plane, "Order");
        assert_eq!(err.chunk_id, "order-1");
    }

    #[test]
    fn mixed_batch_order_query_errors_on_chaos_chunk() {
        // First two are fine; third is a cross-plane violation.
        let chunks = vec![
            make_chunk("ok-1",    "Order"),
            make_chunk("ok-2",    "Bridge"),
            make_chunk("bad-1",   "Chaos"),
            make_chunk("ok-3",    "Order"),
        ];
        let err = guard_chunks(QueryPlane::Order, chunks).unwrap_err();
        assert_eq!(err.chunk_id, "bad-1");
    }

    #[test]
    fn unclassified_bridge_chunk_passes_any_query() {
        let chunk = make_chunk("unclassified", "Bridge");
        assert!(guard_chunk(QueryPlane::Order, &chunk).is_ok());
        assert!(guard_chunk(QueryPlane::Chaos, &chunk).is_ok());
        assert!(guard_chunk(QueryPlane::Bridge, &chunk).is_ok());
    }

    #[test]
    fn guard_chunk_single_row_order_rejects_chaos() {
        let chunk = make_chunk("c1", "Chaos");
        let err = guard_chunk(QueryPlane::Order, &chunk).unwrap_err();
        assert!(err.to_string().contains("query plane is `Order`"));
        assert!(err.to_string().contains("plane `Chaos`"));
    }
}

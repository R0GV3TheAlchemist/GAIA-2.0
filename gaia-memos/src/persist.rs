//! Supabase persistence layer for `gaia-memos`.
//!
//! Handles INSERT and SELECT of [`ChunkRow`] records against the
//! `document_chunks` table via the `postgrest` HTTP client.
//!
//! ## Lexicon plane contract
//!
//! Every [`ChunkRow`] carries `lexicon_plane` (non-null, defaults to
//! `"Bridge"`) and `lexicon_voice` (nullable). These mirror the fields
//! added to `DocumentChunk` in PR #914 and the DB columns added in
//! migration `20260923_gaia20_document_chunks_lexicon_plane`. The retrieve
//! path surfaces both fields so the RAG retrieval guard (follow-on 3)
//! can enforce cross-plane rules without a second round-trip.

use serde::{Deserialize, Serialize};

// ── ChunkRow ─────────────────────────────────────────────────────────────────

/// Database row representation of a `document_chunks` record.
///
/// Fields are kept in `snake_case` to match Postgres column names directly;
/// `serde` handles the mapping from/to JSON for the PostgREST client.
///
/// ## Lexicon fields
///
/// | Field | Column | Nullable | Default |
/// |---|---|---|---|
/// | `lexicon_plane` | `lexicon_plane` | No | `"Bridge"` |
/// | `lexicon_voice` | `lexicon_voice` | Yes | `NULL` |
///
/// Both fields are present in INSERT payloads and SELECT projections.
/// The retrieval guard in `gaia-memos` reads `lexicon_plane` to enforce
/// the Order ↔ Chaos boundary before returning chunks to callers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChunkRow {
    // ── Identity ────────────────────────────────────────────────────────────
    pub id:              String,
    pub document_uri:    String,
    pub document_title:  String,

    // ── Content ─────────────────────────────────────────────────────────────
    pub text:            String,
    pub char_count:      u32,
    pub chunk_index:     u32,
    pub total_chunks:    u32,

    // ── Classification ──────────────────────────────────────────────────────
    pub kind:            String,
    pub domain:          String,
    pub language:        String,
    pub confidence:      String,
    pub access_tier:     String,

    // ── Lexicon plane (PR #914 / migration 20260923) ─────────────────────────
    /// LexiconPlane variant as a string: `"Order"`, `"Chaos"`, or `"Bridge"`.
    /// Never NULL in the DB; defaults to `"Bridge"` until the classify step runs.
    pub lexicon_plane:   String,

    /// LexiconVoice variant as a string, or `None` until classify step resolves.
    /// Maps to the nullable `lexicon_voice` column.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon_voice:   Option<String>,

    // ── Temporal ────────────────────────────────────────────────────────────
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at_unix: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds:     Option<u64>,
}

impl ChunkRow {
    /// Returns `true` if this row has been classified (plane is not Bridge).
    pub fn is_classified(&self) -> bool {
        self.lexicon_plane != "Bridge"
    }

    /// Returns `true` if this row belongs to the Order plane.
    pub fn is_order(&self) -> bool {
        self.lexicon_plane == "Order"
    }

    /// Returns `true` if this row belongs to the Chaos plane.
    pub fn is_chaos(&self) -> bool {
        self.lexicon_plane == "Chaos"
    }
}

// ── SQL column projection ─────────────────────────────────────────────────────

/// Comma-separated column list used in SELECT projections.
///
/// Kept as a constant so the retrieval guard (follow-on 3) and the
/// persist functions reference the same projection without duplication.
pub const CHUNK_COLUMNS: &str = "\
    id,\
    document_uri,\
    document_title,\
    text,\
    char_count,\
    chunk_index,\
    total_chunks,\
    kind,\
    domain,\
    language,\
    confidence,\
    access_tier,\
    lexicon_plane,\
    lexicon_voice,\
    authored_at_unix,\
    ttl_seconds\
";

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn bridge_row() -> ChunkRow {
        ChunkRow {
            id:               "test-id-001".into(),
            document_uri:     "gaia://canon/c156".into(),
            document_title:   "C156 KG Taxonomy".into(),
            text:             "The lexicon separates Order from Chaos.".into(),
            char_count:       38,
            chunk_index:      0,
            total_chunks:     1,
            kind:             "CanonTablet".into(),
            domain:           "lexicon".into(),
            language:         "en".into(),
            confidence:       "Canon".into(),
            access_tier:      "Public".into(),
            lexicon_plane:    "Bridge".into(),
            lexicon_voice:    None,
            authored_at_unix: Some(1_700_000_000),
            ttl_seconds:      None,
        }
    }

    #[test]
    fn bridge_row_is_not_classified() {
        let row = bridge_row();
        assert!(!row.is_classified());
        assert!(!row.is_order());
        assert!(!row.is_chaos());
    }

    #[test]
    fn order_row_is_classified() {
        let mut row = bridge_row();
        row.lexicon_plane = "Order".into();
        row.lexicon_voice = Some("Sacred".into());
        assert!(row.is_classified());
        assert!(row.is_order());
        assert!(!row.is_chaos());
    }

    #[test]
    fn chaos_row_is_classified() {
        let mut row = bridge_row();
        row.lexicon_plane = "Chaos".into();
        row.lexicon_voice = Some("Colloquial".into());
        assert!(row.is_classified());
        assert!(!row.is_order());
        assert!(row.is_chaos());
    }

    #[test]
    fn serde_roundtrip_includes_lexicon_fields() {
        let row = ChunkRow {
            lexicon_plane: "Order".into(),
            lexicon_voice: Some("Scholarly".into()),
            ..bridge_row()
        };
        let json = serde_json::to_string(&row).expect("serialize");
        let back: ChunkRow = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.lexicon_plane, "Order");
        assert_eq!(back.lexicon_voice, Some("Scholarly".into()));
    }

    #[test]
    fn serde_omits_none_lexicon_voice() {
        let row = bridge_row(); // lexicon_voice = None
        let json = serde_json::to_string(&row).expect("serialize");
        // skip_serializing_if = Option::is_none means the key must be absent
        assert!(
            !json.contains("lexicon_voice"),
            "lexicon_voice should be absent from JSON when None"
        );
    }

    #[test]
    fn chunk_columns_contains_lexicon_fields() {
        assert!(CHUNK_COLUMNS.contains("lexicon_plane"));
        assert!(CHUNK_COLUMNS.contains("lexicon_voice"));
    }
}

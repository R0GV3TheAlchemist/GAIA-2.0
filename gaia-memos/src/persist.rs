//! Supabase + SQLite persistence layer for `gaia-memos`.
//!
//! Exposes:
//!  - [`ChunkRow`]      — PostgREST row shape for `document_chunks`
//!  - [`CHUNK_COLUMNS`] — SELECT projection constant
//!  - [`MemStore`]      — SQLite-backed cube store (re-exported from `lib.rs`)
//!  - [`PersistError`]  — error type (re-exported from `lib.rs`)
//!
//! ## Lexicon plane contract
//!
//! Every [`ChunkRow`] carries `lexicon_plane` (non-null, defaults to
//! `"Bridge"`) and `lexicon_voice` (nullable). These mirror the fields
//! added to `DocumentChunk` in PR #914 and the DB columns added in
//! migration `20260923_gaia20_document_chunks_lexicon_plane`. The retrieve
//! path surfaces both fields so the RAG retrieval guard (follow-on 3)
//! can enforce cross-plane rules without a second round-trip.

use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::MemCube;

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

// ── PersistError ──────────────────────────────────────────────────────────────

/// Errors surfaced by [`MemStore`].
#[derive(Debug, Error)]
pub enum PersistError {
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("serialize: {0}")]
    Serialize(#[from] serde_json::Error),
}

// ── MemStore ──────────────────────────────────────────────────────────────────

/// SQLite-backed persistent store for [`MemCube`]s.
///
/// One file on disk, one table (`cubes`), per-user isolation via
/// a `user_did` text column.  All operations are synchronous.
pub struct MemStore {
    conn: Connection,
}

impl MemStore {
    /// Open (or create) a SQLite database at `path`.
    /// Pass `":memory:"` for an ephemeral in-process store.
    pub fn open(path: &str) -> Result<Self, PersistError> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS cubes (
                user_did    TEXT    NOT NULL,
                id          TEXT    NOT NULL,
                cube_type   TEXT    NOT NULL,
                lifecycle   TEXT    NOT NULL,
                content     TEXT    NOT NULL,
                importance  REAL    NOT NULL,
                version     INTEGER NOT NULL,
                access_count INTEGER NOT NULL,
                blob        TEXT    NOT NULL,
                PRIMARY KEY (user_did, id)
            );",
        )?;
        Ok(Self { conn })
    }

    /// Insert or replace a cube for the given user.
    pub fn upsert(&self, user_did: &str, cube: &MemCube) -> Result<(), PersistError> {
        let blob = serde_json::to_string(cube)?;
        self.conn.execute(
            "INSERT INTO cubes \
                (user_did, id, cube_type, lifecycle, content, importance, version, access_count, blob) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9) \
             ON CONFLICT(user_did, id) DO UPDATE SET \
                cube_type=excluded.cube_type, lifecycle=excluded.lifecycle, \
                content=excluded.content, importance=excluded.importance, \
                version=excluded.version, access_count=excluded.access_count, \
                blob=excluded.blob",
            params![
                user_did,
                cube.id.to_string(),
                format!("{:?}", cube.cube_type),
                format!("{:?}", cube.lifecycle),
                &cube.content,
                cube.importance as f64,
                cube.version,
                cube.access_count,
                blob,
            ],
        )?;
        Ok(())
    }

    /// Update only mutable metadata fields (lifecycle, importance,
    /// version, access_count) without rewriting the full blob.
    pub fn update_meta(&self, user_did: &str, cube: &MemCube) -> Result<(), PersistError> {
        self.conn.execute(
            "UPDATE cubes SET lifecycle=?1, importance=?2, version=?3, access_count=?4 \
             WHERE user_did=?5 AND id=?6",
            params![
                format!("{:?}", cube.lifecycle),
                cube.importance as f64,
                cube.version,
                cube.access_count,
                user_did,
                cube.id.to_string(),
            ],
        )?;
        Ok(())
    }

    /// Load all cubes belonging to `user_did`.
    pub fn load_user(&self, user_did: &str) -> Result<Vec<MemCube>, PersistError> {
        let mut stmt = self
            .conn
            .prepare("SELECT blob FROM cubes WHERE user_did=?1")?;
        let rows = stmt.query_map(params![user_did], |row| {
            let blob: String = row.get(0)?;
            Ok(blob)
        })?;
        let mut out = Vec::new();
        for r in rows {
            let blob = r?;
            if let Ok(cube) = serde_json::from_str::<MemCube>(&blob) {
                out.push(cube);
            }
        }
        Ok(out)
    }

    /// Delete a cube by id for the given user.
    pub fn delete(&self, user_did: &str, id: &str) -> SqlResult<usize> {
        self.conn.execute(
            "DELETE FROM cubes WHERE user_did=?1 AND id=?2",
            params![user_did, id],
        )
    }
}

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
        let row = bridge_row();
        let json = serde_json::to_string(&row).expect("serialize");
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

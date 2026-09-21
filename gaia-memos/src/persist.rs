//! [`MemStore`] — SQLite-backed persistence for [`MemCube`]s.
//!
//! Each cube is stored as a JSON blob in the `mem_cubes` table alongside
//! indexed columns for fast tier/lifecycle/importance queries.  The store
//! uses WAL mode so concurrent readers never block a writer.
//!
//! # Schema
//! ```sql
//! mem_cubes(
//!   id           TEXT PRIMARY KEY,   -- UUID as hyphenated string
//!   user_did     TEXT NOT NULL,       -- owner DID (isolation key)
//!   tier         TEXT NOT NULL,       -- Tier debug string
//!   lifecycle    TEXT NOT NULL,       -- Lifecycle debug string
//!   importance   REAL NOT NULL,
//!   version      INTEGER NOT NULL,
//!   blob         TEXT NOT NULL        -- serde_json of MemCube
//! )
//! ```
//!
//! AES-256 encryption at rest is tracked in the follow-up to #724.

use rusqlite::{params, Connection};

use crate::MemCube;

pub type Result<T> = std::result::Result<T, PersistError>;

#[derive(Debug, thiserror::Error)]
pub enum PersistError {
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}

/// SQLite-backed durable store for MemCubes.
pub struct MemStore {
    conn: Connection,
}

impl MemStore {
    /// Open (or create) a MemStore at `path`.
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        // WAL mode: readers never block writers.
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS mem_cubes (
                id         TEXT PRIMARY KEY,
                user_did   TEXT NOT NULL,
                tier       TEXT NOT NULL,
                lifecycle  TEXT NOT NULL,
                importance REAL NOT NULL,
                version    INTEGER NOT NULL,
                blob       TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_user_tier
                ON mem_cubes(user_did, tier);
            CREATE INDEX IF NOT EXISTS idx_importance
                ON mem_cubes(user_did, importance DESC);",
        )?;
        Ok(Self { conn })
    }

    /// Upsert a MemCube (insert or replace on id conflict).
    pub fn upsert(&self, user_did: &str, cube: &MemCube) -> Result<()> {
        let blob = serde_json::to_string(cube)?;
        self.conn.execute(
            "INSERT OR REPLACE INTO mem_cubes
             (id, user_did, tier, lifecycle, importance, version, blob)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                cube.id.to_string(),
                user_did,
                format!("{:?}", cube.tier),
                format!("{:?}", cube.lifecycle),
                cube.importance as f64,
                cube.version as i64,
                blob,
            ],
        )?;
        Ok(())
    }

    /// Load all active cubes for a user into memory.
    pub fn load_user(&self, user_did: &str) -> Result<Vec<MemCube>> {
        let mut stmt = self.conn.prepare(
            "SELECT blob FROM mem_cubes WHERE user_did = ?1",
        )?;
        let cubes = stmt
            .query_map(params![user_did], |row| row.get::<_, String>(0))?
            .filter_map(|r| r.ok())
            .filter_map(|blob| serde_json::from_str::<MemCube>(&blob).ok())
            .collect();
        Ok(cubes)
    }

    /// Delete a cube by id (called from erasure layer after receipt is written).
    pub fn delete(&self, user_did: &str, id: &str) -> Result<usize> {
        let n = self.conn.execute(
            "DELETE FROM mem_cubes WHERE id = ?1 AND user_did = ?2",
            params![id, user_did],
        )?;
        Ok(n)
    }

    /// Persist updated importance / lifecycle columns for an existing cube.
    pub fn update_meta(&self, user_did: &str, cube: &MemCube) -> Result<()> {
        self.conn.execute(
            "UPDATE mem_cubes SET importance = ?1, lifecycle = ?2, version = ?3, blob = ?4
             WHERE id = ?5 AND user_did = ?6",
            params![
                cube.importance as f64,
                format!("{:?}", cube.lifecycle),
                cube.version as i64,
                serde_json::to_string(cube)?,
                cube.id.to_string(),
                user_did,
            ],
        )?;
        Ok(())
    }

    /// Count cubes per tier for a user.
    pub fn count_tier(&self, user_did: &str, tier: &str) -> Result<usize> {
        let n: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM mem_cubes WHERE user_did = ?1 AND tier = ?2",
            params![user_did, tier],
            |row| row.get(0),
        )?;
        Ok(n as usize)
    }
}

//! SQLite episode persist + FTS5 + snapshots (Blueprint 59 Build Part 6).
//! License: Apache-2.0

use rusqlite::{params, Connection, OptionalExtension};

pub struct EpisodeStore {
    conn: Connection,
}

impl EpisodeStore {
    pub fn open(path: &str) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS episodes (
                t_unix_ms INTEGER NOT NULL,
                text TEXT NOT NULL,
                modality TEXT NOT NULL
            )",
            [],
        )?;
        conn.execute(
            "CREATE VIRTUAL TABLE IF NOT EXISTS episodes_fts USING fts5(text)",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS snapshots (
                id TEXT PRIMARY KEY,
                label TEXT NOT NULL,
                current_step TEXT NOT NULL
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn insert(&self, t_unix_ms: i64, text: &str, modality: &str) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO episodes (t_unix_ms, text, modality) VALUES (?1, ?2, ?3)",
            params![t_unix_ms, text, modality],
        )?;
        let rowid = self.conn.last_insert_rowid();
        self.conn.execute(
            "INSERT INTO episodes_fts(rowid, text) VALUES (?1, ?2)",
            params![rowid, text],
        )?;
        Ok(())
    }

    pub fn recent(&self, k: usize) -> rusqlite::Result<Vec<(i64, String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT t_unix_ms, text, modality FROM episodes ORDER BY t_unix_ms DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![k as i64], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        rows.collect()
    }

    pub fn search(&self, vague: &str, k: usize) -> rusqlite::Result<Vec<(i64, String, String)>> {
        let q = vague.trim().replace('"', "");
        if q.is_empty() {
            return self.recent(k);
        }
        let mut stmt = self.conn.prepare(
            "SELECT e.t_unix_ms, e.text, e.modality
             FROM episodes_fts f
             JOIN episodes e ON e.rowid = f.rowid
             WHERE episodes_fts MATCH ?1
             LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![q, k as i64], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        rows.collect()
    }

    pub fn save_snapshot(&self, id: &str, label: &str, current_step: &str) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO snapshots (id, label, current_step) VALUES (?1, ?2, ?3)",
            params![id, label, current_step],
        )?;
        Ok(())
    }

    pub fn find_snapshot(&self, prompt: &str) -> rusqlite::Result<Option<(String, String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, label, current_step FROM snapshots
             WHERE id = ?1 OR label = ?1 OR instr(?1, label) > 0
             ORDER BY rowid DESC LIMIT 1",
        )?;
        stmt.query_row(params![prompt], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .optional()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_survives_reopen() {
        let path = std::env::temp_dir().join("gaia_snap_part6.sqlite");
        let _ = std::fs::remove_file(&path);
        {
            let store = EpisodeStore::open(path.to_str().unwrap()).unwrap();
            store.save_snapshot("snap_0", "destinE memo", "DRAFTING_CARE").unwrap();
        }
        let store = EpisodeStore::open(path.to_str().unwrap()).unwrap();
        let hit = store.find_snapshot("destinE memo").unwrap().unwrap();
        assert_eq!(hit.2, "DRAFTING_CARE");
        let _ = std::fs::remove_file(&path);
    }
}

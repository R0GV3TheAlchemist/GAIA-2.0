//! SQLite episode persist + FTS5 (Blueprint 59 Build Part 5).
//! License: Apache-2.0

use rusqlite::{params, Connection};

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fts_finds_vague_term() {
        let path = std::env::temp_dir().join("gaia_episodes_part5.sqlite");
        let _ = std::fs::remove_file(&path);
        let store = EpisodeStore::open(path.to_str().unwrap()).unwrap();
        store.insert(1, "open CARE.md DestinE draft", "files").unwrap();
        let hits = store.search("CARE", 8).unwrap();
        assert_eq!(hits[0].1.contains("CARE"), true);
        let _ = std::fs::remove_file(&path);
    }
}

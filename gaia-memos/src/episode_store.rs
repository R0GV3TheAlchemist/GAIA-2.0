//! SQLite episode persist — text + time only (Blueprint 59 Build Part 3).
//! No screen/audio capture. License: Apache-2.0

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
        Ok(Self { conn })
    }

    pub fn insert(&self, t_unix_ms: i64, text: &str, modality: &str) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO episodes (t_unix_ms, text, modality) VALUES (?1, ?2, ?3)",
            params![t_unix_ms, text, modality],
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_recent() {
        let path = std::env::temp_dir().join("gaia_episodes_part3.sqlite");
        let _ = std::fs::remove_file(&path);
        let store = EpisodeStore::open(path.to_str().unwrap()).unwrap();
        store.insert(1, "hello", "type").unwrap();
        let rows = store.recent(8).unwrap();
        assert_eq!(rows[0].1, "hello");
        let _ = std::fs::remove_file(&path);
    }
}

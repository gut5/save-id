use rusqlite::{Connection};
use crate::core::Game;

pub struct GameDB {
    conn: Connection,
}

impl GameDB {
    pub fn new(path: &str) -> Self {
        let conn = Connection::open(path).expect("Failed to open DB");
        Self { conn }
    }

    pub fn get_game_by_code(&self, code: &str) -> Option<Game> {
        let mut stmt = self.conn
            .prepare("SELECT code, name, console, region FROM games WHERE code = ?1")
            .ok()?;

        let mut rows = stmt.query([code]).ok()?;

        if let Some(row) = rows.next().ok()? {
            return Some(Game {
                code: row.get(0).ok()?,
                name: row.get(1).ok()?,
                console: row.get(2).ok()?,
                region: row.get(3).ok()?,
            });
        }

        None
    }
}
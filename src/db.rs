use rusqlite::{Connection};
use crate::core::Game;

pub struct GameDB {
    conn: Connection,
}

impl GameDB {
    pub fn new(path: &str) -> Self {
        println!("Opening database: {}", path);

        let conn = Connection::open(path)
            .expect("Failed to open DB");

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM games",
            [],
            |row| row.get(0),
        ).expect("Could not count rows in games table");

        println!("Database contains {} games", count);

        Self { conn }
    }

    pub fn get_game_by_code(&self, code: &str) -> Option<Game> {
        println!("Looking up code: {}", code);

        let mut stmt = self.conn
            .prepare("
                SELECT id, name, type
                FROM games
                WHERE id = ?1
            ")
            .ok()?;

        let mut rows = stmt.query([code]).ok()?;

        if let Some(row) = rows.next().ok()? {
            let game = Game {
                id: row.get(0).ok()?,
                name: row.get(1).ok()?,
                console: row.get(2).ok()?,
            };

            println!("Matched {} -> {}", game.id, game.name);

            return Some(game);
        }

        println!("No match found for {}", code);

        None
    }
}
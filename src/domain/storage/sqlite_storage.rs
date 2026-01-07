#[cfg(not(target_arch = "wasm32"))]
use rusqlite::{params, Connection};

#[cfg(not(target_arch = "wasm32"))]
pub struct SqliteStorage {
    // Añadimos el guion bajo si planeas usarlo luego,
    // o simplemente implementamos un método que la use.
    conn: Connection,
}

#[cfg(not(target_arch = "wasm32"))]
impl SqliteStorage {
    pub fn new(db_path: &str) -> Self {
        let conn = Connection::open(db_path).expect("No se pudo abrir la base de datos");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS user_session (id INTEGER PRIMARY KEY, data TEXT)",
            [],
        )
        .expect("No se pudo crear la tabla de sesión");

        Self { conn }
    }

    // Al añadir un método que USE 'self.conn', el warning desaparecerá automáticamente
    pub fn save_session(&self, json_data: &str) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "INSERT OR REPLACE INTO user_session (id, data) VALUES (1, ?1)",
            params![json_data],
        )?;
        Ok(())
    }

    pub fn get_session(&self) -> Result<String, rusqlite::Error> {
        self.conn
            .query_row("SELECT data FROM user_session WHERE id = 1", [], |row| {
                row.get(0)
            })
    }
}

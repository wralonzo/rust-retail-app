#[cfg(not(target_arch = "wasm32"))]
use crate::domain::models::user::User;
#[cfg(not(target_arch = "wasm32"))]
use crate::domain::storage::storage::SecureStorage;
#[cfg(not(target_arch = "wasm32"))]
use async_trait::async_trait;
#[cfg(not(target_arch = "wasm32"))]
use rusqlite::{Connection};

#[cfg(not(target_arch = "wasm32"))]
pub struct SqliteStorage {
    // Usamos un Mutex interno o abrimos la conexión según sea necesario.
    // Para simplificar y cumplir con Send + Sync, lo manejaremos de forma segura.
    conn: std::sync::Mutex<Connection>,
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

        Self {
            conn: std::sync::Mutex::new(conn),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
impl SecureStorage for SqliteStorage {
    async fn save_session(&self, user: &User) -> Result<(), String> {
        // 1. Clonamos el objeto para no alterar el usuario en memoria
        // Así el HttpClient sigue teniendo el token para las peticiones actuales
        let mut user_to_save = user.clone();

        // 2. Removemos el token antes de serializar a JSON
        user_to_save.token = None;

        // 3. Serializamos la versión "limpia"
        let json = serde_json::to_string(&user_to_save).map_err(|e| e.to_string())?;

        // 4. Guardamos en la base de datos SQLite
        let conn = self.conn.lock().map_err(|_| "Poisoned lock".to_string())?;

        conn.execute(
            "INSERT OR REPLACE INTO user_session (id, data) VALUES (1, ?1)",
            [json], // Usamos la sintaxis moderna de arreglos para params
        )
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_session(&self) -> Result<Option<User>, String> {
        let conn = self.conn.lock().map_err(|_| "Poisoned lock".to_string())?;
        let mut stmt = conn
            .prepare("SELECT data FROM user_session WHERE id = 1")
            .map_err(|e| e.to_string())?;

        let mut rows = stmt.query([]).map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let json: String = row.get(0).map_err(|e| e.to_string())?;
            let user: User = serde_json::from_str(&json).map_err(|e| e.to_string())?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    async fn delete_session(&self) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|_| "Poisoned lock".to_string())?;
        conn.execute("DELETE FROM user_session WHERE id = 1", [])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

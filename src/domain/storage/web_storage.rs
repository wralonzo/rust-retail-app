use crate::domain::storage::storage::SecureStorage;
use async_trait::async_trait;
// SOLUCIÓN A LOS ERRORES E0412:
use crate::domain::models::user::User;

#[cfg(target_arch = "wasm32")]
pub struct WebStorage;

// Marcadores necesarios para usar con Arc dentro de Traits
#[cfg(target_arch = "wasm32")]
unsafe impl Send for WebStorage {}
#[cfg(target_arch = "wasm32")]
unsafe impl Sync for WebStorage {}

#[cfg(target_arch = "wasm32")]
impl WebStorage {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait]
impl SecureStorage for WebStorage {
    async fn save_session(&self, user: &User) -> Result<(), String> {
        let window = web_sys::window().ok_or_else(|| "No window found".to_string())?;
        let storage = window
            .local_storage()
            .map_err(|_| "Storage access denied".to_string())?
            .ok_or_else(|| "LocalStorage not available".to_string())?;

        let json = serde_json::to_string(user).map_err(|e| e.to_string())?;

        storage
            .set_item("user_data", &json)
            .map_err(|_| "Error saving to browser".to_string())
    }

    async fn get_session(&self) -> Result<Option<User>, String> {
        let window = web_sys::window().ok_or("No window")?;
        let storage = window
            .local_storage()
            .map_err(|_| "No storage")?
            .ok_or("No storage")?;

        if let Some(json) = storage.get_item("user_data").map_err(|_| "Read error")? {
            // Aquí se usa User para deserializar
            let user: User = serde_json::from_str(&json).map_err(|e| e.to_string())?;
            return Ok(Some(user));
        }
        Ok(None)
    }

    async fn delete_session(&self) -> Result<(), String> {
        let window = web_sys::window().ok_or("No window")?;
        let storage = window
            .local_storage()
            .map_err(|_| "No storage")?
            .ok_or("No storage")?;

        storage
            .remove_item("user_data")
            .map_err(|_| "Delete error".to_string())
    }
}

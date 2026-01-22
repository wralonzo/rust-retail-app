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
        // 1. Obtener acceso al almacenamiento del navegador
        let window = web_sys::window().ok_or_else(|| "No window found".to_string())?;
        let storage = window
            .local_storage()
            .map_err(|_| "Storage access denied".to_string())?
            .ok_or_else(|| "LocalStorage not available".to_string())?;

        // 2. Crear copia local y anonimizar el token
        // Clonamos aquí para que el 'user' original que recibió la función
        // siga teniendo su token intacto para futuras peticiones HTTP.
        let mut user_to_save = user.clone();
        user_to_save.user.token = None;

        // 3. Serializar a JSON
        let serialized = serde_json::to_string(&user_to_save).map_err(|e| e.to_string())?;

        // 4. Persistir en el navegador
        storage
            .set_item("user_data", &serialized)
            .map_err(|_| "Failed to save to storage".to_string())?;

        Ok(())
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

    async fn save_preference(&self, key: &str, value: &str) -> Result<(), String> {
        let window = web_sys::window().ok_or("No window")?;
        let storage = window.local_storage().map_err(|_| "No storage")?.unwrap();
        storage
            .set_item(key, value)
            .map_err(|_| "Save error".to_string())
    }

    async fn get_preference(&self, key: &str) -> Result<Option<String>, String> {
        let window = web_sys::window().ok_or("No window")?;
        let storage = window.local_storage().map_err(|_| "No storage")?.unwrap();
        storage.get_item(key).map_err(|_| "Read error".to_string())
    }

    async fn delete_preference(&self, key: &str) -> Result<(), String> {
        let window = web_sys::window().ok_or("No window")?;
        let storage = window.local_storage().map_err(|_| "No storage")?.unwrap();
        storage
            .remove_item(key)
            .map_err(|_| "Delete error".to_string())
    }

    async fn save_token(&self, token: &str) -> Result<(), String> {
        // En Web, si quieres persistir el token (bajo tu propio riesgo XSS)
        self.save_preference("auth_token", token).await
    }
}

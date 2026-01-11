use crate::domain::models::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait SecureStorage: Send + Sync {
    // Sesión
    async fn save_session(&self, user: &User) -> Result<(), String>;
    async fn get_session(&self) -> Result<Option<User>, String>;
    async fn delete_session(&self) -> Result<(), String>;

    // Preferencias
    async fn save_preference(&self, key: &str, value: &str) -> Result<(), String>;
    async fn get_preference(&self, key: &str) -> Result<Option<String>, String>;
    async fn delete_preference(&self, key: &str) -> Result<(), String>;

    // Token (Opcional: para storage seguro separado)
    async fn save_token(&self, token: &str) -> Result<(), String>;
}

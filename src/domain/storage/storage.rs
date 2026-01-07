use crate::domain::models::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait SecureStorage: Send + Sync {
    async fn save_session(&self, user: &User) -> Result<(), String>;
    async fn get_session(&self) -> Result<Option<User>, String>;
    async fn delete_session(&self) -> Result<(), String>;
}

use std::sync::Arc;
use crate::domain::models::errors::AppError;
use crate::infrastructure::client::client_repository::ClientRepositoryTrait;

pub struct DeleteClientUseCase {
    repository: Arc<dyn ClientRepositoryTrait>,
}

impl DeleteClientUseCase {
    pub fn new(repository: Arc<dyn ClientRepositoryTrait>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id:i32) -> Result<String, AppError> {
        self.repository.delete(id).await
    }
}
use std::sync::Arc;
use crate::domain::models::client::client_response::ClientResponse;
use crate::domain::models::errors::AppError;
use crate::infrastructure::client::client_repository::ClientRepositoryTrait;

pub struct FindOneClientUseCase {
    repository: Arc<dyn ClientRepositoryTrait>,
}

impl FindOneClientUseCase {
    pub fn new(repository: Arc<dyn ClientRepositoryTrait>) -> Self{
        Self { repository }
    }

    pub async fn execute(&self, id: i32) -> Result<ClientResponse, AppError> {
        self.repository.find_one(id).await
    }
}
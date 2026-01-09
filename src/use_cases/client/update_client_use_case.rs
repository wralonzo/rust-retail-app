use std::sync::Arc;
use crate::domain::models::client::client_response::{ClientRequest, ClientResponse};
use crate::domain::models::errors::AppError;
use crate::infrastructure::client::client_repository::ClientRepositoryTrait;

pub struct UpdateClientUseCase {
    repository: Arc<dyn ClientRepositoryTrait>,
}

impl UpdateClientUseCase {
    pub fn new(repository: Arc<dyn ClientRepositoryTrait>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32, req: ClientRequest) -> Result<ClientResponse, AppError> {
        self.repository.update(id, req).await
    }
}
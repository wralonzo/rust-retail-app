use std::sync::Arc;
use crate::domain::models::client::client_response::{ClientRequest, ClientResponse};
use crate::domain::models::errors::AppError;
use crate::infrastructure::client::client_repository::ClientRepositoryTrait;

pub struct AddClientUseCase {
    repository: Arc<dyn ClientRepositoryTrait>,
}
impl AddClientUseCase {
    pub fn new(
        repository:Arc<dyn ClientRepositoryTrait>,
    ) -> Self {
        Self {
            repository,
        }
    }

    pub async fn execute(&self, req: ClientRequest) -> Result<ClientResponse, AppError> {
      self.repository.save(req).await
    }
}
use std::sync::Arc;
use crate::domain::models::client::client_response::ClientResponse;
use crate::domain::models::errors::AppError;
use crate::domain::models::responses::PaginatedResponse;
use crate::infrastructure::client::client_repository::ClientRepositoryTrait;

pub struct FindClientUseCase {
    repository:Arc<dyn ClientRepositoryTrait>,
}

impl FindClientUseCase{
    pub fn new(repository: Arc<dyn ClientRepositoryTrait>) -> Self{
        Self { repository }
    }

    pub async fn execute(&self, search: String, sort: String, page: u32, size: u32, client_type: String) -> Result<PaginatedResponse<ClientResponse>, AppError> {
        let query_params =  format!("?term={}&sort={}&page={}&size={}&clientType={}",
            search, sort, page, size, client_type
        );
        self.repository.find(query_params).await
    }
}

use std::sync::Arc;
use crate::bridge::main_bridge::AppContainer;
use super::ClientBridge;
use crate::domain::models::client::client_response::{ClientRequest, ClientResponse};
use crate::domain::models::errors::AppError;

#[derive(uniffi::Record)]
pub struct ClientPaginatedResponse {
    pub content: Vec<ClientResponse>,
    pub total_elements: u64,
    pub total_pages: u32,
    pub size: u32,
    pub number: u32,
    pub last: bool,
    pub first: bool,
}

#[uniffi::export]
impl ClientBridge {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        let container = AppContainer::get_instance();
        Arc::new(Self::new_internal(container))
    }

    pub async fn get_clients(&self, search: String, sort: String, page: u32, size: u32, client_type: String) -> Result<ClientPaginatedResponse, AppError> {
        let internal_data = self.find_client_use_case.execute(search, sort, page, size, client_type).await?;
        Ok(ClientPaginatedResponse {
            content: internal_data.content,
            total_elements: internal_data.total_elements,
            total_pages: internal_data.total_pages,
            size: internal_data.size,
            number: internal_data.number,
            last: internal_data.last,
            first: internal_data.first,
        })

    }

    pub async fn save_client(&self, req: ClientRequest) -> Result<ClientResponse, AppError> {
        self.add_client_use_case.execute(req).await
    }

    pub async fn find_one_client(&self, id: i32) -> Result<ClientResponse, AppError> {
        self.find_one_client_use_case.execute(id).await
    }

    pub async fn delete_client(&self, id: i32) -> Result<String, AppError> {
        self.delete_client_use_case.execute(id).await
    }

    pub async fn update_client(&self, id: i32, req: ClientRequest) -> Result<ClientResponse, AppError> {
        self.update_client_use_case.execute(id, req).await
    }
}
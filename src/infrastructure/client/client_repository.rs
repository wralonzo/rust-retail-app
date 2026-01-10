use crate::domain::models::errors::AppError;
use crate::domain::models::responses::{PaginatedResponse};
use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::models::client::client_response::{ClientRequest, ClientResponse};
use send_wrapper::SendWrapper;
use crate::infrastructure::http_client_rust::HttpClientRust;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ClientRepositoryTrait: Send + Sync {
    /* ?term=&sort=name,asc&page=0&size=10&clientType */
    async fn find(&self, query_params: String) -> Result<PaginatedResponse<ClientResponse>, AppError>;
    async fn save(&self, req: ClientRequest) -> Result<ClientResponse, AppError>;
    async fn find_one(&self, id: i32) -> Result<ClientResponse, AppError>;
    async fn delete(&self, id: i32) -> Result<String, AppError>;
    async fn update(&self, id: i32, req: ClientRequest) -> Result<ClientResponse, AppError>;
}

pub struct ClientRepository {
    api: Arc<HttpClientRust>,
}

// Las funciones que no pertenecen al Trait (como new) van en un impl propio
impl ClientRepository {
    const CONTROLLER_PATH: &'static str = "/client";

    pub fn new(api: Arc<HttpClientRust>) -> Self {
        Self { api }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ClientRepositoryTrait for ClientRepository {
    async fn find(&self, query_params: String) -> Result<PaginatedResponse<ClientResponse>, AppError> {
        let endpoint = format!("{}{}", Self::CONTROLLER_PATH, query_params);

        // El método .get ya sabe que O: PaginatedResponse<ClientResponse>
        let future = self.api.get::<PaginatedResponse<ClientResponse>>(&endpoint);

        SendWrapper::new(future).await
    }

    async fn save(&self, req: ClientRequest) -> Result<ClientResponse, AppError> {
        // Usamos el método .post directo que creamos en HttpClientRust
        let future = self.api.post::<ClientRequest, ClientResponse>(
            Self::CONTROLLER_PATH,
            req
        );

        SendWrapper::new(future).await
    }

    async fn find_one(&self, id: i32) -> Result<ClientResponse, AppError> {
        let endpoint = format!("{}/{}", Self::CONTROLLER_PATH, id);

        let future = self.api.get::<ClientResponse>(&endpoint);

        SendWrapper::new(future).await
    }

    async fn delete(&self, id: i32) -> Result<String, AppError> {
        let endpoint = format!("{}/{}", Self::CONTROLLER_PATH, id);

        // Si el delete devuelve un String de confirmación
        let future = self.api.delete::<String>(&endpoint);

        SendWrapper::new(future).await
    }

    async fn update(&self, id: i32, req: ClientRequest) -> Result<ClientResponse, AppError> {
        let endpoint = format!("{}/{}", Self::CONTROLLER_PATH, id);

        let future = self.api.put::<ClientRequest, ClientResponse>(
            &endpoint,
            req
        );

        SendWrapper::new(future).await
    }
}
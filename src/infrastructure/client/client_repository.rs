use crate::domain::models::errors::AppError;
use crate::domain::models::responses::{PaginatedResponse};
use crate::infrastructure::api_service::ApiService;
use async_trait::async_trait;
use reqwest::Method;
use std::sync::Arc;
use crate::domain::models::client::client_response::{ClientRequest, ClientResponse};
use send_wrapper::SendWrapper;

#[async_trait]
pub trait ClientRepositoryTrait: Send + Sync {
    /* ?term=&sort=name,asc&page=0&size=10&clientType */
    async fn find(&self, query_params: String) -> Result<PaginatedResponse<ClientResponse>, AppError>;
    async fn save(&self, req: ClientRequest) -> Result<ClientResponse, AppError>;
    async fn find_one(&self, id: i32) -> Result<ClientResponse, AppError>;
    async fn delete(&self, id: i32) -> Result<String, AppError>;
    async fn update(&self, id: i32, req: ClientRequest) -> Result<ClientResponse, AppError>;
}

pub struct ClientRepository {
    api: Arc<ApiService>,
}

// Las funciones que no pertenecen al Trait (como new) van en un impl propio
impl ClientRepository {
    const CONTROLLER_PATH: &'static str = "/client";

    pub fn new(api: Arc<ApiService>) -> Self {
        Self { api }
    }
}

#[async_trait]
impl ClientRepositoryTrait for ClientRepository {
    async fn find(&self, query_params: String) -> Result<PaginatedResponse<ClientResponse>, AppError> {
        let endpoint = format!("{}{}", Self::CONTROLLER_PATH, query_params);

        // 1. Apuntamos al envoltorio HttpResponseApi
        let future = self.api.request_one::<(), PaginatedResponse<ClientResponse>>(
            Method::GET,
            &endpoint,
            None,
            None
        );

        // 2. Ejecutamos el futuro
        let response = SendWrapper::new(future).await?;

        // 3. Extraemos solo la data (que es el PaginatedResponse)
        Ok(response)
    }

    async fn save(&self, req: ClientRequest) -> Result<ClientResponse, AppError> {
        let future = self.api.request_one::<ClientRequest, ClientResponse>(
            Method::POST,
            Self::CONTROLLER_PATH,
            Some(req),
            None,
        );

        // 2. Lo envolvemos para "engañar" a UniFFI y decirle que es Send
        SendWrapper::new(future).await
    }

    async fn find_one(&self, id: i32) -> Result<ClientResponse, AppError> {
        let api = self.api.clone();
        let endpoint = format!("{}/{}", Self::CONTROLLER_PATH, id);

        // CREAMOS EL WRAPPER QUE CONTIENE TODA LA LÓGICA ASÍNCRONA
        let wrapped_fut = SendWrapper::new(async move {
            api.request_one::<(), ClientResponse>(
                reqwest::Method::GET,
                &endpoint,
                None,
                None,
            ).await
        });

        // Al hacer await al wrapper, UniFFI queda satisfecho porque el wrapper es 'Send'
        wrapped_fut.await
    }

    async fn delete(&self, id: i32) -> Result<String, AppError> {
        let endpoint = format!("{}/{}",Self::CONTROLLER_PATH, id); // Evita E0716
        let future = self.api.request_one::<(), String>(
            Method::DELETE,
            &endpoint,
            None,
            None,
        );
        SendWrapper::new(future).await
    }

    async fn update(&self, id:i32, req: ClientRequest) -> Result<ClientResponse, AppError> {
        let endpoint = format!("{}/{}/delete",Self::CONTROLLER_PATH, id);
        let future = self.api.request_one::<ClientRequest, ClientResponse>(
            Method::PUT,
            &endpoint,
            Some(req),
            None,
        );

        // SendWrapper "convierte" el futuro !Send en Send
        SendWrapper::new(future).await
    }
}
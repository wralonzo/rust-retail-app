use std::sync::Arc;
// src/domain/use_cases/get_google_config_use_case.rs
use crate::{
    domain::models::{errors::AppError, google_id::GoogleClientId},
};
use crate::infrastructure::http_client_rust::HttpClientRust;

pub struct GetGoogleConfigUseCase {
    http: Arc<HttpClientRust>,
}

impl GetGoogleConfigUseCase {
    pub fn new(http: Arc<HttpClientRust>) -> Self {
        Self {
            http,
        }
    }

    pub async fn execute(&self) -> Result<GoogleClientId, AppError> {
        // Si tu do_get ahora devuelve Result<O, AppError>
        let response: GoogleClientId = self
            .http
            .get("/config/google-client-id") // Ajusta el endpoint según tu API
            .await?;

        Ok(response)
    }
}

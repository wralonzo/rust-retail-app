use std::sync::Arc;
// src/domain/use_cases/get_google_config_use_case.rs
use crate::{
    domain::models::{errors::AppError, google_id::GoogleClientId},
    infrastructure::api_service::ApiService,
};

pub struct GetGoogleConfigUseCase {
    api_service: Arc<ApiService>,
}

impl GetGoogleConfigUseCase {
    pub fn new(api_service: Arc<ApiService>) -> Self {
        Self {
            api_service,
        }
    }

    pub async fn execute(&self) -> Result<GoogleClientId, AppError> {
        // Si tu do_get ahora devuelve Result<O, AppError>
        let response: GoogleClientId = self
            .api_service
            .do_get("/config/google-client-id") // Ajusta el endpoint según tu API
            .await?;

        Ok(response)
    }
}

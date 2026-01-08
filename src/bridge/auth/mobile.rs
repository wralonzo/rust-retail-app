use super::AuthBridge;
use crate::domain::models::errors::AppError;
use crate::domain::models::google_response::GoogleResponse;
use crate::domain::models::user::User;
use crate::use_cases::get_google_config_use_case::GetGoogleConfigUseCase;
use std::sync::Arc;

#[derive(uniffi::Record)]
pub struct UserPaginatedResponse {
    pub content: Vec<User>,
    pub total_elements: u64,
    pub total_pages: u32,
    pub size: u32,
    pub number: u32,
    pub last: bool,
    pub first: bool,
}

#[uniffi::export]
impl AuthBridge {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            use_case: Arc::new(Self::build_use_case()),
            google_use_case: Arc::new(GetGoogleConfigUseCase::new()), // <--- Faltaba este campo
        })
    }

    pub async fn login(&self, email: String, password: String) -> Result<User, AppError> {
        self.internal_login(email, password).await
    }

    pub async fn get_id_google_client(&self) -> Result<GoogleResponse, AppError> {
        let client_id_model = self.internal_get_google_client().await?;

        // Mapeamos el resultado genérico al tipo concreto que UniFFI acepta
        Ok(GoogleResponse {
            data: client_id_model,
            success: true,
            message: Some("Configuración cargada correctamente".to_string()),
        })
    }

    pub async fn get_users(&self, page: u32) -> Result<UserPaginatedResponse, AppError> {
        let internal_data = self.internal_fetch_users(page).await?;

        // Simplemente envolvemos el genérico en el struct concreto
        Ok(UserPaginatedResponse {
            content: internal_data.content,
            total_elements: internal_data.total_elements,
            total_pages: internal_data.total_pages,
            size: internal_data.size,
            number: internal_data.number,
            last: internal_data.last,
            first: internal_data.first,
        })
    }
}

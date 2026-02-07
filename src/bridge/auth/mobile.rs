use super::AuthBridge;
use crate::bridge::main_bridge::AppContainer;
use crate::domain::models::errors::AppError;
use crate::domain::models::google_response::GoogleResponse;
use crate::domain::models::user::User;
use send_wrapper::SendWrapper;
use std::sync::Arc;

#[derive(uniffi::Record)]
pub struct UserPaginatedResponse {
    pub content: Vec<User>,
    pub total_elements: u32,
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
        let container = AppContainer::get_instance();
        Arc::new(Self::new_internal(container))
    }

    pub async fn login(&self, email: String, password: String) -> Result<User, AppError> {
        // Envolvemos el futuro interno en SendWrapper
        SendWrapper::new(self.internal_login(email, password)).await
    }

    pub async fn get_id_google_client(&self) -> Result<GoogleResponse, AppError> {
        let future = self.internal_get_google_client();
        let client_id_model = SendWrapper::new(future).await?;

        Ok(GoogleResponse {
            data: client_id_model,
            success: true,
            message: Some("Configuración cargada correctamente".to_string()),
        })
    }

    pub async fn login_google(&self, google_app_id: String) -> Result<User, AppError> {
        SendWrapper::new(self.internal_login_google(google_app_id)).await
    }

    pub async fn logout(&self) {
        // Al ser un void (unit type), también lo envolvemos si es async
        SendWrapper::new(AppContainer::get_instance().logout()).await;
    }

    pub async fn get_user_local(&self) -> Result<Option<User>, AppError> {
        let container = AppContainer::get_instance();

        // Envolvemos la llamada al container en SendWrapper
        SendWrapper::new(container.get_user_local()).await
    }
}

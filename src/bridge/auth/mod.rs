use crate::bridge::main_bridge::AppContainer;
use crate::domain::models::errors::AppError;
use crate::domain::models::google_id::GoogleClientId;
use crate::domain::models::login_request::LoginRequest;
use crate::domain::models::user::User;
use crate::use_cases::auth_user_use_case::LoginUseCase;
use crate::use_cases::get_google_config_use_case::GetGoogleConfigUseCase;
use std::sync::Arc;

use crate::infrastructure::auth_repository::AuthRepository;

#[cfg(not(target_arch = "wasm32"))]
pub mod mobile;
#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Object))]
pub struct AuthBridge {
    pub(crate) login_use_case: Arc<LoginUseCase>,
    pub(crate) google_use_case: Arc<GetGoogleConfigUseCase>,
}

impl AuthBridge {
    pub fn new_internal(container: &AppContainer) -> Self {
        // 1. Usamos el nuevo http_client centralizado
        let auth_repo = Arc::new(AuthRepository::new(container.http_client.clone()));

        // 2. Pasamos el http_client al caso de uso para que pueda hacer 'set_token'
        let login_use_case_internal = Arc::new(LoginUseCase::new(
            auth_repo,
            container.storage.clone(),
            container.http_client.clone(), // Inyección vital
        ));

        let google_use_case = Arc::new(GetGoogleConfigUseCase::new(container.http_client.clone()));

        Self {
            login_use_case: login_use_case_internal,
            google_use_case,
        }
    }

    pub(crate) async fn internal_login(
        &self,
        user: String,
        pass: String,
    ) -> Result<User, AppError> {
        let request = LoginRequest {
            username: user,
            password: pass,
        };
        self.login_use_case.execute(request).await
    }

    pub(crate) async fn internal_get_google_client(&self) -> Result<GoogleClientId, AppError> {
        self.google_use_case.execute().await
    }

    pub(crate) async fn internal_login_google(
        &self,
        app_google_id: String,
    ) -> Result<User, AppError> {
        self.login_use_case
            .execute_login_google(app_google_id)
            .await
    }
}

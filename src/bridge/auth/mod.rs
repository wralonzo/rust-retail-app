use crate::domain::models::errors::AppError;
use crate::domain::models::google_id::GoogleClientId;
use crate::domain::models::login_request::LoginRequest;
use crate::domain::models::responses::PaginatedResponse;
use crate::domain::models::user::User;
use crate::use_cases::auth::LoginUseCase;
use crate::use_cases::get_google_config_use_case::GetGoogleConfigUseCase;
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
pub mod mobile;
#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Object))]
pub struct AuthBridge {
    pub(crate) use_case: Arc<LoginUseCase>,
    pub(crate) google_use_case: Arc<GetGoogleConfigUseCase>,
}

impl AuthBridge {
    pub(crate) fn build_use_case() -> LoginUseCase {
        let api_service = crate::infrastructure::api_service::ApiService::new();
        let auth_repo = crate::infrastructure::auth_repository::AuthRepository::new(api_service);
        LoginUseCase::new(auth_repo)
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
        self.use_case.execute(request).await
    }

    pub(crate) async fn internal_get_google_client(&self) -> Result<GoogleClientId, AppError> {
        self.google_use_case.execute().await
    }

    pub(crate) async fn internal_fetch_users(&self, page: u32) -> Result<PaginatedResponse<User>, AppError> {
        self.use_case.get_all_users(page).await
    }
}

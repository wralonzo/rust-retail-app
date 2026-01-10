use std::sync::Arc;

use crate::domain::models::errors::AppError;
use crate::domain::models::login_request::LoginRequest;
use crate::domain::models::responses::PaginatedResponse;
use crate::domain::models::user::User;
use async_trait::async_trait;
use serde_json::json;
use crate::infrastructure::http_client_rust::HttpClientRust;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait AuthRepositoryTrait: Send + Sync {
    async fn login(&self, req: LoginRequest) -> Result<User, AppError>;
}
pub struct AuthRepository {
    http: Arc<HttpClientRust>,
}

impl AuthRepository {
    pub fn new(http: Arc<HttpClientRust>) -> Self {
        Self { http }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl AuthRepositoryTrait for AuthRepository   {
    async fn login(&self, req: LoginRequest) -> Result<User, AppError> {
        self.http
            .post::<LoginRequest, User>("/auth/login", req)
            .await
    }
}

impl AuthRepository {
    pub async fn fetch_users(&self, page: u32) -> Result<PaginatedResponse<User>, AppError> {
        let endpoint = format!("/users?page={}", page);

        // En HttpClientRust: get<O>(path)
        // El tipo de salida (O) debe coincidir con el Result
        self.http
            .get::<PaginatedResponse<User>>(&endpoint)
            .await
    }

    pub async fn login_google(&self, google_app_id: String) -> Result<User, AppError> {
        // Usamos json! para el cuerpo dinámico
        let body = json!({
            "idToken": google_app_id
        });

        self.http
            .post::<serde_json::Value, User>("/auth/google", body)
            .await
    }
}
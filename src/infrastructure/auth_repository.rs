use std::sync::Arc;

use crate::domain::models::errors::AppError;
use crate::domain::models::login_request::LoginRequest;
use crate::domain::models::responses::PaginatedResponse;
use crate::domain::models::user::User;
use crate::infrastructure::api_service::ApiService;
use async_trait::async_trait;
use reqwest::Method;
use serde_json::json;

#[async_trait]
pub trait AuthRepositoryTrait: Send + Sync {
    async fn login(&self, req: LoginRequest) -> Result<User, AppError>;
}
pub struct AuthRepository {
    api: Arc<ApiService>,
}

impl AuthRepository {
    pub fn new(api: Arc<ApiService>) -> Self {
        Self { api }
    }

    pub async fn login(&self, req: LoginRequest) -> Result<User, AppError> {
        self.api
            .request_one::<LoginRequest, User>(Method::POST, "/auth/login", Some(req), None)
            .await
    }

    pub async fn fetch_users(&self, page: u32) -> Result<PaginatedResponse<User>, AppError> {
        let endpoint = format!("/users?page={}", page);

        // El ApiService ya sabe desempaquetar el wrapper JSON y devolvernos la PaginatedResponse
        self.api
            .request_paginated::<(), User>(reqwest::Method::GET, &endpoint, None)
            .await
    }

    pub async fn login_google(&self, google_app_id: String) -> Result<User, AppError> {
        // Creamos el cuerpo dinámicamente sin definir una struct
        let body = json!({
            "idToken": google_app_id
        });
        self.api.request_one::<serde_json::Value, User>(
            reqwest::Method::POST,
            "/auth/google",
            Some(body),
            None,
        )
        .await
    }
}

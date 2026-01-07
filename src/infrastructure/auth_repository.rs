use async_trait::async_trait;
use crate::domain::models::errors::AppError;
use crate::domain::models::login_request::LoginRequest;
use crate::domain::models::responses::PaginatedResponse;
use crate::domain::models::user::User;
use crate::infrastructure::api_service::ApiService;
use reqwest::Method;


#[async_trait]
pub trait AuthRepositoryTrait: Send + Sync {
    async fn login(&self, req: LoginRequest) -> Result<User, AppError>;
}
pub struct AuthRepository {
    api: ApiService,
}

impl AuthRepository {
    pub fn new(api: ApiService) -> Self {
        Self { api }
    }

    pub async fn login(&self, req: LoginRequest) -> Result<User, AppError> {
        // Usamos request_one porque devuelve HttpResponseApiFindOne<User>
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
}

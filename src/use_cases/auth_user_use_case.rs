use std::sync::Arc;

use crate::domain::models::errors::AppError;
use crate::domain::models::login_request::LoginRequest;
use crate::domain::models::responses::PaginatedResponse; // Importamos el modelo de paginación
use crate::domain::models::user::User;
use crate::domain::storage::storage::SecureStorage;
use crate::infrastructure::api_service::ApiService;
use crate::infrastructure::auth_repository::AuthRepository;

pub struct LoginUseCase {
    repository: Arc<AuthRepository>, // Cambiado a Arc<dyn ...>
    storage: Arc<dyn SecureStorage>, // Cambiado a Arc<dyn ...>
    pub api_service: Arc<ApiService>,
}

impl LoginUseCase {
    pub fn new(
        repository: Arc<AuthRepository>,
        storage: Arc<dyn SecureStorage>,
        api_service: Arc<ApiService>,
    ) -> Self {
        Self {
            repository,
            storage,
            api_service,
        }
    }

    /// Lógica de autenticación
    pub async fn execute(&self, req: LoginRequest) -> Result<User, AppError> {
        // 1. Validaciones de negocio
        if req.username.is_empty() {
            return Err(AppError::EmptyField {
                message: "Usuario".into(),
            }); // Usamos el error semántico que definimos antes
        }

        if req.password.is_empty() {
            return Err(AppError::EmptyField {
                message: "Contraseña".into(),
            }); // Usamos el error semántico que definimos antes
        }

        if !req.username.contains('@') {
            return Err(AppError::EmailInvalid);
        }

        let user_login = self.repository.login(req).await?;

        // CORRECCIÓN 1: save_session es async, necesita .await antes del ?
        // Además, mapeamos el String de error a AppError
        self.storage
            .save_session(&user_login)
            .await
            .map_err(|e| AppError::ParseError { message: e })?;

        // CORRECCIÓN 2: No existe 'token', es 'user_login.token'
        let token = user_login.token.clone()
            .ok_or(AppError::AuthError {
                message: "No se recibió un token del servidor".to_string()
            })?;
        // Y quitamos el '?' porque set_token no devuelve un Result
        self.api_service.set_token(token);

        Ok(user_login)
    }

    /// Lógica para obtener la lista de usuarios paginada
    pub async fn get_all_users(&self, page: u32) -> Result<PaginatedResponse<User>, AppError> {
        self.repository.fetch_users(page).await
    }

    pub async fn init_session(&self) -> bool {
        if let Ok(Some(user)) = self.storage.get_session().await {
            self.api_service.set_token(user.token);
            return true;
        }
        false
    }

    pub async fn execute_logout(&self) -> Result<(), String> {
        self.storage.delete_session().await?;
        self.api_service.clear_token();
        Ok(())
    }

    pub async fn execute_login_google(&self, app_google_id: String) -> Result<User, AppError> {
        if app_google_id.is_empty() {
            return Err(AppError::EmptyField {
                message: "Módulo de google no configurado".into(),
            });
        }

        let user_login_google = self.repository.login_google(app_google_id).await?;
        self.storage
            .save_session(&user_login_google)
            .await
            .map_err(|e| AppError::ParseError { message: e })?;

        let token = user_login_google.token.clone()
            .ok_or(AppError::AuthError {
                message: "No se recibió un token del servidor".to_string()
            })?;
        // Y quitamos el '?' porque set_token no devuelve un Result
        self.api_service.set_token(token);

        Ok(user_login_google)
    }
}

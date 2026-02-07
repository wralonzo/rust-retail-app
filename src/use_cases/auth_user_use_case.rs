use std::sync::Arc;

use crate::domain::models::errors::AppError;
use crate::domain::models::login_request::LoginRequest;
use crate::domain::models::user::User;
use crate::domain::storage::storage::SecureStorage;
use crate::infrastructure::auth_repository::{AuthRepository, AuthRepositoryTrait};
use crate::infrastructure::http_client_rust::HttpClientRust;

pub struct LoginUseCase {
    repository: Arc<AuthRepository>, // Cambiado a Arc<dyn ...>
    storage: Arc<dyn SecureStorage>, // Cambiado a Arc<dyn ...>
    http: Arc<HttpClientRust>,
}

impl LoginUseCase {
    pub fn new(
        repository: Arc<AuthRepository>,
        storage: Arc<dyn SecureStorage>,
        http: Arc<HttpClientRust>,
    ) -> Self {
        Self {
            repository,
            storage,
            http,
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

        // 2. Intento de Login contra el Repositorio (API)
        let user_login = self.repository.login(req).await?;

        // 3. Manejo de Credenciales y Persistencia
        if let Some(token) = &user_login.user.token {
            // A. Memoria RAM: Para peticiones inmediatas
            self.http.set_token(token.clone());

            // B. DB Local (Preferencia): Guardamos el token para 'init_session'
            // Usamos save_token que definimos en el SecureStorage
            self.storage
                .save_token(token)
                .await
                .map_err(|e| AppError::DatabaseError { message: e })?;

            // C. DB Local (Sesión): Guardamos el perfil (sin token por seguridad)
            self.storage
                .save_session(&user_login)
                .await
                .map_err(|e| e.to_string())
                .map_err(|e| AppError::DatabaseError { message: e })?;
        }

        Ok(user_login)
    }

    pub async fn init_session(&self) -> bool {
        // 1. Intentamos recuperar el token guardado como una preferencia
        if let Ok(Some(token)) = self.storage.get_preference("auth_token").await {
            // 2. Inyectamos el token en el cliente HTTP para futuras peticiones
            self.http.set_token(token);

            // 3. Opcional: Intentamos cargar también los datos del usuario
            // para que la UI tenga el nombre y foto desde el inicio
            if let Ok(Some(user)) = self.storage.get_session().await {
                // Aquí podrías actualizar un estado interno de 'currentUser' si lo tienes
                log::info!(
                    "Sesión restaurada para el usuario: {}",
                    user.profile.username
                );
            }

            return true;
        }

        log::info!("No se encontró sesión previa.");
        false
    }

    pub async fn execute_logout(&self) -> Result<(), String> {
        self.storage.delete_session().await?;
        self.storage.delete_preference("auth_token").await?;
        self.http.clear_token();
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

        let token = user_login_google
            .user
            .token
            .clone()
            .ok_or(AppError::AuthError {
                message: "No se recibió un token del servidor".to_string(),
            })?;
        // Y quitamos el '?' porque set_token no devuelve un Result
        self.http.set_token(token);

        Ok(user_login_google)
    }
}

use crate::domain::models::errors::AppError;
use crate::domain::models::login_request::LoginRequest;
use crate::domain::models::responses::PaginatedResponse; // Importamos el modelo de paginación
use crate::domain::models::user::User;
use crate::infrastructure::auth_repository::AuthRepository;

pub struct LoginUseCase {
    repository: AuthRepository,
}

impl LoginUseCase {
    pub fn new(repository: AuthRepository) -> Self {
        Self { repository }
    }

    /// Lógica de autenticación
    pub async fn execute(&self, req: LoginRequest) -> Result<User, AppError> {
        // 1. Validaciones de negocio
        if req.username.is_empty() || req.password.is_empty() {
            return Err(AppError::BadRequest); // Usamos el error semántico que definimos antes
        }

        if !req.username.contains('@') {
            return Err(AppError::NetworkError {
                message: "Formato de email inválido".into(),
            });
        }

        // 2. Llamada al repositorio
        self.repository.login(req).await
    }

    /// Lógica para obtener la lista de usuarios paginada
    pub async fn get_all_users(&self, page: u32) -> Result<PaginatedResponse<User>, AppError> {
        // Aquí podrías agregar validaciones, por ejemplo, que la página no sea negativa
        // aunque al ser u32 ya nos aseguramos de que sea 0 o mayor.

        self.repository.fetch_users(page).await
    }
}

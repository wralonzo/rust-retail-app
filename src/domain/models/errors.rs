#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)] // Esto ayuda a simplificar la exportación
pub enum AppError {
    #[error("Network error: {message}")]
    NetworkError { message: String }, // Cambiado de (String) a { message: String }

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Complete los campos")]
    BadRequest,

    #[error("Conflicto con el recurso")]
    Conflict,

    #[error("Error desconocido")]
    Unknown,
}

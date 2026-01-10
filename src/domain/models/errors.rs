#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ts_rs::TS, uniffi::Record)]
#[ts(export)]
pub struct ApiErrorPayload {
    /// JSON crudo del backend convertido a String para compatibilidad total
    pub error_api: String,
    pub message: String,
    pub code: i16,
}

#[derive(Debug, thiserror::Error, uniffi::Error, serde::Serialize, serde::Deserialize, ts_rs::TS)]
#[uniffi(flat_error)]
#[ts(export)]
#[serde(tag = "type", content = "payload")]
pub enum AppError {
    #[error("API Error: {0:?}")]
    ApiError(ApiErrorPayload),

    #[error("Error de red: {message}")]
    NetworkError { message: String },

    #[error("Error de autenticación: {message}")]
    AuthError { message: String },

    #[error("No encontrado: {message}")]
    NotFoundError { message: String },

    #[error("Pago requerido: {message}")]
    PaymentRequired { message: String },

    #[error("Error en servidor: {message}")]
    ServerError { message: String },

    #[error("Error de procesamiento: {message}")]
    ParseError { message: String },

    #[error("El campo {message} está vacío")]
    EmptyField { message: String },

    #[error("No autorizado")]
    Unauthorized,

    #[error("Solicitud incorrecta")]
    BadRequest,

    #[error("Correo inválido")]
    EmailInvalid,

    #[error("Conflicto: {message}")]
    Conflict { message: String },

    #[error("Almacenamiento local, no compatible, {message}")]
    DatabaseError { message: String },

    #[error("Error desconocido")]
    Unknown,
}

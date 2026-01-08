#[derive(Debug, thiserror::Error, uniffi::Error, serde::Serialize, serde::Deserialize)]
#[uniffi(flat_error)]
pub enum AppError {
    #[error("Error de red: {message}")]
    NetworkError { message: String },

    #[error("{message}")]
    AuthError { message: String },

    #[error("{message}")]
    NotFoundError { message: String },

    #[error("{message}")]
    PaymentRequired { message: String },

    #[error(" {message}")]
    ServerError { message: String },

    #[error("{message}")]
    ParseError { message: String },

    #[error("El campo: {message} está vacío")]
    EmptyField { message: String },

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Complete todos los campos")]
    BadRequest,

    #[error("El correo ingresado es incorrecto")]
    EmailInvalid,

    #[error("{message}")]
    Conflict { message: String },

    #[error("Error desconocido")]
    Unknown,
}

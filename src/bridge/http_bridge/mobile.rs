use super::GenericHttpBridge;
use crate::domain::models::errors::AppError;
use send_wrapper::SendWrapper;
use std::sync::Arc;

#[uniffi::export]
impl GenericHttpBridge {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        let container = crate::bridge::main_bridge::AppContainer::get_instance();
        Arc::new(Self::new_internal(container))
    }

    pub async fn get_json(&self, path: String) -> Result<String, AppError> {
        let future = self.internal_get(path);
        let result = SendWrapper::new(future).await?;
        Ok(result.to_string())
    }

    pub async fn post_json(&self, path: String, body_json: String) -> Result<String, AppError> {
        let body: serde_json::Value =
            serde_json::from_str(&body_json).map_err(|e| AppError::ParseError {
                message: e.to_string(),
            })?;

        let future = self.internal_post(path, body);
        let result = SendWrapper::new(future).await?;
        Ok(result.to_string())
    }

    pub async fn path_json(&self, path: String, body_json: String) -> Result<String, AppError> {
        let body: serde_json::Value =
            serde_json::from_str(&body_json).map_err(|err| AppError::ParseError {
                message: err.to_string(),
            })?;

        let future = self.internal_patch(path, body);
        let result = SendWrapper::new(future).await?;
        Ok(result.to_string())
    }

    pub async fn delete_json(&self, path: String) -> Result<String, AppError> {
        let future = self.internal_delete(path);
        let result = SendWrapper::new(future).await?;
        Ok(result.to_string())
    }

    // --- MÉTODOS DE ARCHIVOS (UPLOAD & DOWNLOAD) ---

    /// Sube un archivo al servidor. Devuelve la respuesta del servidor como JSON String.
    pub async fn upload_file(
        &self,
        endpoint: String,
        bytes: Vec<u8>,
        file_name: String,
        content_type: String,
    ) -> Result<String, AppError> {
        let future = self.internal_upload_file(&endpoint, bytes, file_name, content_type);
        let result = SendWrapper::new(future).await?;
        Ok(result.to_string())
    }

    /// Descarga un archivo, obtiene su nombre del backend y lo guarda en la carpeta especificada.
    /// Devuelve la RUTA LOCAL absoluta donde se guardó el archivo.
    pub async fn download_file(&self, endpoint: String) -> Result<String, AppError> {
        // Usamos la implementación inteligente que extrae el nombre del Header
        let future = self
        .internal_download_file(endpoint, "./app_data/documents");
        let local_path = SendWrapper::new(future).await?;
        Ok(local_path)
    }

    /// Lógica para abrir el archivo usando el visor nativo del Sistema Operativo (iOS/Android)
    pub async fn open_file_externally(&self, path: String) -> Result<(), AppError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            opener::open(std::path::Path::new(&path)).map_err(|e| AppError::ServerError {
                message: format!("No se pudo abrir el archivo nativamente: {}", e),
            })?;
            Ok(())
        }
        #[cfg(target_arch = "wasm32")]
        Ok(())
    }
}

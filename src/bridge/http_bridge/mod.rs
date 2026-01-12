use crate::bridge::main_bridge::AppContainer;
use crate::domain::models::errors::AppError;
use crate::infrastructure::http_repository::HttpRepository;
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
pub mod mobile;
#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Object))]
pub struct GenericHttpBridge {
    pub(crate) repository: Arc<HttpRepository>,
}

impl GenericHttpBridge {
    pub fn new_internal(container: &AppContainer) -> Self {
        // Usamos el http_repository que registramos en el AppContainer
        Self {
            repository: container.http_repository.clone(),
        }
    }

    // Métodos internos que usan serde_json::Value para ser "comodines"
    pub(crate) async fn internal_get(&self, path: String) -> Result<serde_json::Value, AppError> {
        self.repository.get(&path).await
    }

    pub(crate) async fn internal_post(
        &self,
        path: String,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, AppError> {
        self.repository.post(&path, body).await
    }

    pub(crate) async fn internal_patch(
        &self,
        path: String,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, AppError> {
        self.repository.patch(&path, body).await
    }

    pub(crate) async fn internal_delete(
        &self,
        path: String,
    ) -> Result<serde_json::Value, AppError> {
        self.repository.delete(&path).await
    }

    pub(crate) async fn internal_upload_file(
        &self,
        endpoint: &str,
        bytes: Vec<u8>,
        file_name: String,
        content_type: String,

    ) -> Result<serde_json::Value, AppError> {
        // Asegúrate de que el repositorio devuelva serde_json::Value
        self.repository
            .upload_file::<serde_json::Value>(endpoint, bytes, file_name, content_type)
            .await
    }

    pub(crate) async fn internal_download_file(
        &self,
        endpoint: String,
        folder: &str,
    ) -> Result<String, AppError> {
        // El repositorio devuelve la ruta (String), no un JSON
        self.repository
            .download_and_store_document(endpoint, folder)
            .await
    }
}

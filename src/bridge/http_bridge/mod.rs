use std::sync::Arc;
use crate::bridge::main_bridge::AppContainer;
use crate::domain::models::errors::AppError;
use crate::infrastructure::http_repository::HttpRepository;

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
        body: serde_json::Value
    ) -> Result<serde_json::Value, AppError> {
        self.repository.post(&path, body).await
    }
}
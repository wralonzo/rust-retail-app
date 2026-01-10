use super::GenericHttpBridge;
use crate::domain::models::errors::AppError;
use std::sync::Arc;
use send_wrapper::SendWrapper;

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
        let body: serde_json::Value = serde_json::from_str(&body_json)
            .map_err(|e| AppError::ParseError { message: e.to_string() })?;

        let future = self.internal_post(path, body);
        let result = SendWrapper::new(future).await?;
        Ok(result.to_string())
    }
}
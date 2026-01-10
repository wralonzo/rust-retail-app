use std::sync::Arc;
use crate::domain::models::errors::AppError;
use crate::infrastructure::http_repository::HttpRepository;

pub struct GetResourceUseCase {
    repository: Arc<HttpRepository>,
}

impl GetResourceUseCase {
    pub async fn execute<O>(&self, path: &str) -> Result<O, AppError>
    where O: serde::de::DeserializeOwned + ts_rs::TS {
        self.repository.get
            ::<O>(path).await
    }
}
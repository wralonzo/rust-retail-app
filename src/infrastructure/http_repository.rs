use std::sync::Arc;
use crate::domain::models::errors::AppError;
use crate::infrastructure::http_client_rust::HttpClientRust;

pub struct HttpRepository {
    // Compartimos el mismo HttpClient que tiene el RwLock del Token
    api: Arc<HttpClientRust>,
}

impl HttpRepository {
    pub fn new(api: Arc<HttpClientRust>) -> Self {
        Self { api }
    }

    pub async fn get<O>(&self, path: &str) -> Result<O, AppError>
    where O: serde::de::DeserializeOwned {
        self.api.get::<O>(path).await
    }

    pub async fn post<I, O>(&self, path: &str, body: I) -> Result<O, AppError>
    where I: serde::Serialize, O: serde::de::DeserializeOwned{
        self.api.post::<I, O>(path, body).await
    }

    pub async fn delete<O>(&self, path: &str) -> Result<O, AppError>
    where O: serde::de::DeserializeOwned {
        self.api.delete::<O>(path).await
    }

    pub async fn patch<I, O>(&self, path: &str, body: I) -> Result<O, AppError>
    where
        I: serde::Serialize, // <--- AÑADIR ESTO
        O: serde::de::DeserializeOwned
    {
        self.api.patch::<I, O>(path, body).await
    }

    // Agrega put, patch, etc., siguiendo el mismo patrón
}
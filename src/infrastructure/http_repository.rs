use crate::domain::models::errors::AppError;
use crate::infrastructure::http_client_rust::HttpClientRust;
use std::sync::Arc;

pub struct HttpRepository {
    // Compartimos el mismo HttpClient que tiene el RwLock del Token
    api: Arc<HttpClientRust>,
}

impl HttpRepository {
    pub fn new(api: Arc<HttpClientRust>) -> Self {
        Self { api }
    }

    pub async fn get<O>(&self, path: &str) -> Result<O, AppError>
    where
        O: serde::de::DeserializeOwned,
    {
        self.api.get::<O>(path).await
    }

    pub async fn post<I, O>(&self, path: &str, body: I) -> Result<O, AppError>
    where
        I: serde::Serialize,
        O: serde::de::DeserializeOwned,
    {
        self.api.post::<I, O>(path, body).await
    }

    pub async fn delete<O>(&self, path: &str) -> Result<O, AppError>
    where
        O: serde::de::DeserializeOwned,
    {
        self.api.delete::<O>(path).await
    }

    pub async fn patch<I, O>(&self, path: &str, body: I) -> Result<O, AppError>
    where
        I: serde::Serialize, // <--- AÑADIR ESTO
        O: serde::de::DeserializeOwned,
    {
        self.api.patch::<I, O>(path, body).await
    }

    pub async fn upload_file<O>(
        &self,
        endpoint: &str,
        bytes: Vec<u8>,
        file_name: String,
        content_type: String,
    ) -> Result<O, AppError>
    where
        O: serde::de::DeserializeOwned,
    {
        self.api
            .upload_multipart::<O>(endpoint, bytes, file_name, content_type)
            .await
    }

    pub async fn download_and_store_document(
        &self,
        endpoint: String,
        folder: &str,
    ) -> Result<String, AppError> {
        // El cliente ahora se encarga de:
        // 1. Preguntar al backend el nombre
        // 2. Descargar bytes
        // 3. Si es web, devolver Base64. Si es nativo, devolver Path.
        self.api.download_file_smart(&endpoint, &folder).await
    }
}

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
        file_name: String,
        endpoint: String,
        folder: &str,
    ) -> Result<String, AppError> {
        // En un entorno real, es mejor obtener el path de datos de la app
        // mediante una configuración inyectada, pero esto funciona para desarrollo:

        if let Err(e) = std::fs::create_dir_all(folder) {
            return Err(AppError::ServerError {
                message: format!("Error creando directorio: {}", e),
            });
        }

        let full_path = format!("{}/{}", folder, file_name);

        // 2. Ejecutar descarga delegando al cliente HTTP
        self.api.download_to_local(&endpoint, &full_path).await
    }
}

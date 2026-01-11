use crate::domain::models::errors::{ApiErrorPayload, AppError};
use crate::domain::models::responses::HttpResponseObject;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub struct HttpClientRust {
    client: reqwest::Client,
    token: Arc<RwLock<Option<String>>>,
    global_headers: HashMap<String, String>,
    base_url: String,
}

impl HttpClientRust {
    pub fn new(base_url: String, initial_headers: HashMap<String, String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            token: Arc::new(RwLock::new(None)),
            global_headers: initial_headers,
        }
    }

    //Envio de token
    pub fn set_token<T: Into<Option<String>>>(&self, token: T) {
        if let Ok(mut t) = self.token.write() {
            let token_value: Option<String> = token.into();

            match token_value {
                Some(val) => {
                    log::info!("🔑 Token de sesión actualizado");
                    *t = Some(val);
                }
                None => {
                    log::warn!("🗑️ El token recibido es nulo, limpiando sesión");
                    *t = None;
                }
            }
        }
    }

    //delete token
    pub fn clear_token(&self) {
        if let Ok(mut t) = self.token.write() {
            *t = None;
        }
    }

    // No necesita ser async
    fn build_request(&self, method: Method, url: &str) -> reqwest::RequestBuilder {
        // 1. Iniciamos el builder con el cliente
        let mut builder = self
            .client
            .request(method.clone(), url)
            .header("x-app-origin", "TopFashion-Angular-App");

        // 2. Leemos el token de la memoria (RwLock es síncrono)
        if let Ok(token_guard) = self.token.read() {
            if let Some(token) = token_guard.as_ref() {
                // Solo inyectamos si el token existe
                builder = builder.header("Authorization", format!("Bearer {}", token));
            } else {
                // Esto es normal en endpoints públicos como /login o /register
                log::info!("ℹ️ Petición pública (sin token) hacia: {}", url);
            }
        } else {
            log::error!("❌ Error crítico: RwLock envenenado (poisoned)");
        }

        // 3. Devolvemos el builder listo para ser ejecutado con .send().await
        builder
    }

    fn logout_internal(&self) {
        if let Ok(mut token_guard) = self.token.write() {
            *token_guard = None;
        }
    }

    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T, AppError>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();

        // 1. Manejo de Sesión Expirada
        if status == reqwest::StatusCode::UNAUTHORIZED {
            self.logout_internal();
            return Err(AppError::Unauthorized);
        }

        // 2. Leemos los bytes UNA SOLA VEZ para poder usarlos varias veces
        let bytes = response.bytes().await.map_err(|e| AppError::NetworkError {
            message: e.to_string(),
        })?;

        // 3. Si el status NO es éxito, procesamos el JSON de error de Spring
        if !status.is_success() {
            // 1. Parsear el JSON completo del backend
            let error_json: serde_json::Value = serde_json::from_slice(&bytes)
                .unwrap_or(serde_json::json!({ "message": "Error desconocido en el servidor" }));

            // 2. Prioridad de Mensaje: message (amigable) > exception (técnico)
            let friendly_message = error_json["message"]
                .as_str()
                .or_else(|| error_json["exception"].as_str())
                .unwrap_or("Ocurrió un error inesperado en el servidor")
                .to_string();

            // 3. Guardar el JSON crudo como String en error_api para trazabilidad
            let raw_api_error =
                serde_json::to_string(&error_json).unwrap_or_else(|_| status.to_string());

            return Err(AppError::ApiError(ApiErrorPayload {
                error_api: raw_api_error, // Aquí irá todo el JSON: { "status": 403, "exception": "DisabledException", ... }
                message: friendly_message, // "Esta cuenta ha sido desactivada..."
                code: status.as_u16() as i16,
            }));
        }

        // 4. Si es éxito, decodificamos el wrapper HttpResponseObject<T>
        let wrapper: HttpResponseObject<T> =
            serde_json::from_slice(&bytes).map_err(|e| AppError::ParseError {
                message: format!(
                    "Error decodificando JSON: {}. Body: {}",
                    e,
                    String::from_utf8_lossy(&bytes)
                ),
            })?;

        Ok(wrapper.data)
    }

    //Logica para metodos genericos
    // 2. El método ÚNICO principal
    /// Motor central de peticiones
    pub async fn request<I, O>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<I>,
    ) -> Result<O, AppError>
    where
        I: Serialize,
        O: DeserializeOwned, // Restricción necesaria para HttpResponseObject
    {
        let url = format!("{}{}", self.base_url, endpoint);

        // Usamos build_request para centralizar Headers y Auth
        let mut rb = self.build_request(method, &url);

        // Inyectar headers configurados en el init (además de los estándar)
        for (key, value) in &self.global_headers {
            rb = rb.header(key, value);
        }

        if let Some(b) = body {
            rb = rb.json(&b);
        }

        let response = rb.send().await.map_err(|e| AppError::NetworkError {
            message: e.to_string(),
        })?;

        // Interceptor 401 y mapeo de errores/éxito
        self.handle_response(response).await
    }

    // --- MÉTODOS DE CONVENIENCIA ---

    pub async fn get<O>(&self, path: &str) -> Result<O, AppError>
    where
        O: DeserializeOwned,
    {
        self.request::<(), O>(Method::GET, path, None).await
    }

    pub async fn post<I, O>(&self, path: &str, body: I) -> Result<O, AppError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        self.request(Method::POST, path, Some(body)).await
    }

    pub async fn put<I, O>(&self, path: &str, body: I) -> Result<O, AppError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        self.request(Method::PUT, path, Some(body)).await
    }

    pub async fn patch<I, O>(&self, path: &str, body: I) -> Result<O, AppError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        self.request(Method::PATCH, path, Some(body)).await
    }

    pub async fn delete<O>(&self, path: &str) -> Result<O, AppError>
    where
        O: DeserializeOwned,
    {
        self.request::<(), O>(Method::DELETE, path, None).await
    }

    //Upload file
    pub async fn upload_multipart<O>(
        &self,
        endpoint: &str,
        bytes: Vec<u8>,
        file_name: String,
        content_type: String,
    ) -> Result<O, AppError>
    where
        O: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);

        // 1. Usamos tu motor central de construcción de peticiones
        // Esto inyecta automáticamente el Token y los Global Headers
        let rb = self.build_request(Method::POST, &url);

        // 2. Construir el formulario multipart
        let file_part = reqwest::multipart::Part::bytes(bytes)
            .file_name(file_name)
            .mime_str(&content_type)
            .map_err(|e| AppError::ParseError {
                message: format!("Error en metadatos del archivo: {}", e),
            })?;

        let form = reqwest::multipart::Form::new().part("file", file_part);

        // 3. Ejecutar la petición
        let response = rb
            .multipart(form)
            .send()
            .await
            .map_err(|e| AppError::NetworkError {
                message: e.to_string(),
            })?;

        // 4. Usamos tu interceptor central de respuestas
        // Esto maneja el 401, parsea el JSON de error de Spring y extrae la data del wrapper
        self.handle_response(response).await
    }

    pub async fn download_to_local(
        &self,
        endpoint: &str,
        save_path: &str,
    ) -> Result<String, AppError> {
        let url = format!("{}{}", self.base_url, endpoint);

        // 1. Construir petición con Auth y Headers globales
        let rb = self.build_request(reqwest::Method::GET, &url);

        let response = rb.send().await.map_err(|e| AppError::NetworkError {
            message: e.to_string(),
        })?;

        // 2. Si hay error (400, 401, 500), delegamos a handle_response
        // Usamos () como tipo esperado porque solo queremos que procese el error
        if !response.status().is_success() {
            let _ = self.handle_response::<()>(response).await?;
            return Err(AppError::ServerError {
                message: "Error inesperado tras validación".into(),
            });
        }

        // 3. Descarga de flujo de bytes
        let bytes = response.bytes().await.map_err(|e| AppError::NetworkError {
            message: format!("Error al descargar contenido: {}", e),
        })?;

        // 4. Escritura directa a FileSystem
        // Nota: El directorio ya debe existir (el Repositorio se encarga de eso)
        std::fs::write(save_path, &bytes).map_err(|e| AppError::ServerError {
            message: format!("Error de escritura en disco: {}", e),
        })?;

        log::info!("✅ Descarga exitosa: {}", save_path);
        Ok(save_path.to_string())
    }
}

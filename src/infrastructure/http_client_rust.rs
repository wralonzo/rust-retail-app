use std::collections::HashMap;
use std::sync::Arc;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::RwLock;
use crate::domain::models::errors::{ApiErrorPayload, AppError};
use crate::domain::models::responses::HttpResponseObject;


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
    pub fn  set_token<T: Into<Option<String>>>(&self, token: T) {
        if let Ok(mut t) = self.token.write() {
            let token_value: Option<String> = token.into();

            match token_value {
                Some(val) => {
                    log::info!("🔑 Token de sesión actualizado");
                    *t = Some(val);
                },
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
        let mut builder = self.client
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
            log::warn!("🚫 Sesión expirada (401). Limpiando token...");
            self.logout_internal();
            return Err(AppError::Unauthorized);
        }

        // 2. Leemos los bytes UNA SOLA VEZ para poder usarlos varias veces
        let bytes = response.bytes().await.map_err(|e| AppError::NetworkError {
            message: e.to_string(),
        })?;

        // 3. Si el status NO es éxito, procesamos el JSON de error de Spring
        if !status.is_success() {
            // Intentamos parsear el JSON de error que viene en los bytes
            let error_json: serde_json::Value = serde_json::from_slice(&bytes)
                .unwrap_or(serde_json::json!({ "message": "Error desconocido en el servidor" }));

            // Extraemos el mensaje (Spring usa el campo "message")
            let message = error_json["exception"]
                .as_str()
                .or_else(|| error_json["error"].as_str())
                .unwrap_or("No se pudo obtener el detalle del error")
                .to_string();

            return Err(AppError::ApiError(ApiErrorPayload {
                error_api: status.to_string(),
                message, // "For input string: A4340D"
                code: status.as_u16() as i16
            }));
        }

        // 4. Si es éxito, decodificamos el wrapper HttpResponseObject<T>
        let wrapper: HttpResponseObject<T> = serde_json::from_slice(&bytes).map_err(|e| {
            AppError::ParseError {
                message: format!("Error decodificando JSON: {}. Body: {}", e, String::from_utf8_lossy(&bytes)),
            }
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
    where O: DeserializeOwned {
        self.request::<(), O>(Method::GET, path, None).await
    }

    pub async fn post<I, O>(&self, path: &str, body: I) -> Result<O, AppError>
    where I: Serialize, O: DeserializeOwned  {
        self.request(Method::POST, path, Some(body)).await
    }

    pub async fn put<I, O>(&self, path: &str, body: I) -> Result<O, AppError>
    where I: Serialize, O: DeserializeOwned  {
        self.request(Method::PUT, path, Some(body)).await
    }

    pub async fn patch<I, O>(&self, path: &str, body: I) -> Result<O, AppError>
    where I: Serialize, O: DeserializeOwned  {
        self.request(Method::PATCH, path, Some(body)).await
    }

    pub async fn delete<O>(&self, path: &str) -> Result<O, AppError>
    where O: DeserializeOwned  {
        self.request::<(), O>(Method::DELETE, path, None).await
    }
}
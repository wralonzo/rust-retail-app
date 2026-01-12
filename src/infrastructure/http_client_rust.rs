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
    pub base_url: String,
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

        // 1. Manejo de Sesión Expirada (401)
        // Lo manejamos ANTES de intentar leer cualquier cuerpo, porque Spring
        // a menudo no envía JSON en este caso específico.
        if status == reqwest::StatusCode::UNAUTHORIZED {
            log::error!("🚫 401 Unauthorized detectado. Limpiando sesión local.");
            self.logout_internal();
            return Err(AppError::Unauthorized);
        }

        // 2. Intentar leer los bytes de la respuesta
        // Usamos un mapeo de error más descriptivo para diferenciar fallo de RED vs Servidor
        let bytes = response.bytes().await.map_err(|e| AppError::NetworkError {
            message: format!("Error de conexión al leer respuesta: {}", e),
        })?;

        // 3. Si el status NO es éxito (4xx o 5xx)
        if !status.is_success() {
            // Intentar parsear el JSON de error de Spring si existe
            let error_json: serde_json::Value =
                serde_json::from_slice(&bytes).unwrap_or_else(|_| {
                    // Si no es JSON, devolvemos un objeto genérico con el status
                    serde_json::json!({
                        "message": format!("Error del servidor (Status {})", status.as_u16()),
                        "status": status.as_u16()
                    })
                });

            let friendly_message = error_json["message"]
                .as_str()
                .or_else(|| error_json["exception"].as_str())
                .unwrap_or("Ocurrió un error inesperado en el servidor")
                .to_string();

            let raw_api_error =
                serde_json::to_string(&error_json).unwrap_or_else(|_| status.to_string());

            return Err(AppError::ApiError(ApiErrorPayload {
                error_api: raw_api_error,
                message: friendly_message,
                code: status.as_u16() as i16,
            }));
        }

        // 4. Si es éxito, decodificamos el wrapper HttpResponseObject<T>
        let wrapper: HttpResponseObject<T> = serde_json::from_slice(&bytes).map_err(|e| {
            // Log para depuración si el backend cambia el formato
            let body_str = String::from_utf8_lossy(&bytes);
            log::error!("❌ Error de parseo: {}. Body recibido: {}", e, body_str);

            AppError::ParseError {
                message: format!("Error decodificando JSON de éxito: {}", e),
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

    // En infrastructure/http_client_rust.rs

    pub async fn download_file_smart(
        &self,
        endpoint: &str,
        _folder: &str,
    ) -> Result<String, AppError> {
        let url = format!("{}{}", self.base_url, endpoint);
        let rb = self.build_request(reqwest::Method::GET, &url);
        let response = rb.send().await.map_err(|e| AppError::NetworkError {
            message: e.to_string(),
        })?;

        if !response.status().is_success() {
            let _ = self.handle_response::<()>(response).await?;
            return Err(AppError::ServerError {
                message: "Error en descarga".into(),
            });
        }

        // --- 1. EXTRAER INFORMACIÓN DE LOS HEADERS PRIMERO (Sin mover 'response') ---

        // Extraer Nombre del archivo
        let _file_name = response
            .headers()
            .get(reqwest::header::CONTENT_DISPOSITION)
            .and_then(|val| val.to_str().ok())
            .and_then(|s| {
                s.split(';')
                    .find(|part| part.trim().starts_with("filename="))
                    .map(|p| p.trim().replace("filename=", "").replace("\"", ""))
            })
            .unwrap_or_else(|| "archivo_descargado".to_string());

        // Extraer MIME Type (lo guardamos en un String para usarlo después del move)
        let mime_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        // --- 2. CONSUMIR LA RESPUESTA (Aquí 'response' desaparece) ---
        let bytes = response.bytes().await.map_err(|e| AppError::NetworkError {
            message: e.to_string(),
        })?;

        // --- 3. LÓGICA HÍBRIDA ---

        #[cfg(target_arch = "wasm32")]
        {
            use base64::{engine::general_purpose, Engine as _};
            let b64 = general_purpose::STANDARD.encode(&bytes);
            // Usamos mime_type aquí, así que no hay warning en WASM
            Ok(format!("data:{};base64,{}", mime_type, b64))
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // En NATIVO (Mobile/Desktop), mime_type no se usa, así que lo "silenciamos"
            let _ = mime_type;

            std::fs::create_dir_all(_folder).ok();
            let full_path = format!("{}/{}", _folder, _file_name);
            std::fs::write(&full_path, &bytes).map_err(|e| AppError::ServerError {
                message: e.to_string(),
            })?;
            Ok(full_path)
        }
    }
}

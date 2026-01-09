use crate::config::config::get_base_url;
use crate::domain::models::errors::AppError;
use crate::domain::models::responses::{HttpResponseApi, HttpResponseObject, PaginatedResponse};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::RwLock;

pub struct ApiService {
    client: reqwest::Client,
    token: RwLock<Option<String>>,
}

impl ApiService {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();

        // 2. Insertar el header requerido por tu backend
        headers.insert(
            "x-app-origin",
            HeaderValue::from_static("TopFashion-Angular-App"),
        );

        // 3. (Opcional) Asegurar que siempre enviamos JSON
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Self {
            client: reqwest::Client::builder()
                .default_headers(headers) // Configuración global
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()), // Fallback por si falla el builder
            token: RwLock::new(None),
        }
    }

    pub fn get_base_url_internal(&self) -> String {
        get_base_url()
    }

    // Método para guardar el token después del login
    pub fn set_token(&self, token: String) {
        log::info!("Intentando login para el usuario: {}", token);
        if let Ok(mut t) = self.token.write() {
            *t = Some(token);
        }
    }

    pub fn clear_token(&self) {
        if let Ok(mut t) = self.token.write() {
            *t = None;
        }
    }

    // Función interna para construir la petición con el Bearer Token
    fn build_request(&self, method: reqwest::Method, url: &str) -> reqwest::RequestBuilder {
        let mut rb = self.client.request(method.clone(), url);

        match self.token.read() {
            Ok(t_guard) => {
                if let Some(token) = t_guard.as_ref() {
                    log::info!("✅ [ApiService] Inyectando Token en {}: Bearer ...{}", method, &token[..token.len().min(10)]);
                    rb = rb.header("Authorization", format!("Bearer {}", token));
                } else {
                    log::warn!("⚠️ [ApiService] El guard del token es None (No hay sesión activa) para {}", url);
                }
            },
            Err(e) => {
                log::error!("❌ [ApiService] Error de bloqueo (Poisoned Lock) al leer el token: {}", e);
            }
        }
        rb
    }

    fn process_error_data(status: reqwest::StatusCode, error_body: Option<serde_json::Value>) -> AppError {
        let backend_message = error_body
            .and_then(|json| {
                json.get("message")
                    .and_then(|m| m.as_str().map(|s| s.to_string()))
            })
            .unwrap_or_else(|| format!("Error HTTP: {}", status));

        match status.as_u16() {
            401 => AppError::AuthError { message: backend_message },
            404 => AppError::NotFoundError { message: backend_message },
            409 => AppError::Conflict { message: backend_message },
            500..=599 => AppError::ServerError { message: backend_message },
            _ => AppError::NetworkError { message: backend_message },
        }
    }
    pub async fn request_one<I, O>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<I>,
        query_params: Option<&[(&str, &str)]>,
    ) -> Result<O, AppError>
    where
        I: Serialize,
        O: DeserializeOwned + ts_rs::TS,
    {
        let url = format!("{}{}", self.get_base_url_internal(), endpoint);
        let mut rb = self.build_request(method.clone(), &url);

        if let Some(params) = query_params { rb = rb.query(params); }
        if method != Method::GET && method != Method::HEAD {
            if let Some(b) = body { rb = rb.json(&b); }
        }

        // 1. Enviamos y esperamos la respuesta inicial
        let response = rb.send().await.map_err(|e| AppError::NetworkError {
            message: e.to_string(),
        })?;

        let status = response.status();

        // 2. SOLUCIÓN RADICAL: Mover la respuesta a un futuro que solo devuelva Bytes
        // Al usar un bloque async separado o simplemente llamar a bytes(),
        // evitamos que 'response' sea capturada en el estado de 'request_one'.
        let full_bytes = response.bytes().await.map_err(|e| AppError::ParseError {
            message: e.to_string(),
        })?;

        // A partir de aquí, 'response' ya no existe, solo 'full_bytes' (que es Send)
        if !status.is_success() {
            let error_json = serde_json::from_slice::<serde_json::Value>(&full_bytes).ok();
            return Err(Self::process_error_data(status, error_json));
        }

        serde_json::from_slice::<HttpResponseObject<O>>(&full_bytes)
            .map(|wrapper| wrapper.data)
            .map_err(|e| AppError::ParseError {
                message: format!("Error decoding object: {}", e),
            })
    }

    pub async fn request_paginated<I, O>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<I>,
    ) -> Result<PaginatedResponse<O>, AppError>
    where
        I: Serialize,
        O: DeserializeOwned + ts_rs::TS,
    {
        let url = format!("{}{}", self.get_base_url_internal(), endpoint);
        let mut rb = self.build_request(method, &url);
        if let Some(b) = body { rb = rb.json(&b); }

        // --- BLOQUE DE ALCANCE CRÍTICO ---
        let (status, full_bytes) = {
            let response = rb.send().await.map_err(|e| AppError::NetworkError {
                message: e.to_string(),
            })?;

            let s = response.status();
            let b = response.bytes().await.map_err(|e| AppError::ParseError {
                message: e.to_string(),
            })?;
            (s, b)
        };

        if !status.is_success() {
            let error_json = serde_json::from_slice::<serde_json::Value>(&full_bytes).ok();
            return Err(Self::process_error_data(status, error_json));
        }

        serde_json::from_slice::<HttpResponseApi<O>>(&full_bytes)
            .map(|wrapper| wrapper.data)
            .map_err(|e| AppError::ParseError {
                message: format!("Error decoding pagination: {}", e),
            })
    }

    pub async fn do_get<O: serde::de::DeserializeOwned + ts_rs::TS>(
        &self,
        endpoint: &str,
    ) -> Result<O, AppError> {
        self.request_one::<(), O>(reqwest::Method::GET, endpoint, None, None)
            .await
    }
}

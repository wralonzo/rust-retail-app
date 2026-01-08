use crate::config::config::get_base_url;
use crate::domain::models::errors::AppError;
use crate::domain::models::responses::{HttpResponseApi, HttpResponseObject, PaginatedResponse};
use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

pub struct ApiService {
    client: reqwest::Client,
}

impl ApiService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn get_base_url_internal(&self) -> String {
        get_base_url()
    }

    async fn handle_error_response(response: reqwest::Response) -> AppError {
        let status = response.status();

        // Intentamos extraer el JSON de error
        let error_body = response.json::<serde_json::Value>().await.ok();

        let backend_message = error_body
            .and_then(|json| {
                json.get("message")
                    .and_then(|m| m.as_str().map(|s| s.to_string()))
            })
            .unwrap_or_else(|| format!("Error HTTP: {}", status));

        match status.as_u16() {
            401 => AppError::AuthError {
                message: backend_message,
            },
            404 => AppError::NotFoundError {
                message: backend_message,
            },
            402 => AppError::PaymentRequired {
                message: backend_message,
            },
            409 => AppError::Conflict {
                message: backend_message,
            },
            500..=599 => AppError::ServerError {
                message: backend_message,
            },
            _ => AppError::NetworkError {
                message: backend_message,
            },
        }
    }

    pub async fn request_one<I, O>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        body: Option<I>,
        query_params: Option<&[(&str, &str)]>,
    ) -> Result<O, AppError>
    where
        I: serde::Serialize,
        O: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.get_base_url_internal(), endpoint);
        let mut rb = self.client.request(method.clone(), &url);

        if let Some(params) = query_params {
            rb = rb.query(params);
        }
        if method != reqwest::Method::GET && method != reqwest::Method::HEAD {
            if let Some(b) = body {
                rb = rb.json(&b);
            }
        }

        let response = rb.send().await.map_err(|e| AppError::NetworkError {
            message: e.to_string(),
        })?;

        // CLAVE: Verificar status ANTES de consumir el JSON
        let status = response.status();
        if !status.is_success() {
            return Err(Self::handle_error_response(response).await);
        }

        let wrapper = response
            .json::<HttpResponseObject<O>>()
            .await
            .map_err(|e| AppError::ParseError {
                message: format!("Error decoding object: {}", e),
            })?;

        Ok(wrapper.data)
    }

    pub async fn request_paginated<I, O>(
        &self,
        method: Method, // Cambiado de _method a method para usarlo
        endpoint: &str,
        body: Option<I>,
    ) -> Result<PaginatedResponse<O>, AppError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let url = format!("{}{}", self.get_base_url_internal(), endpoint);

        // Usamos el constructor genérico para soportar POST paginados si fuera necesario
        let mut rb = self.client.request(method, &url);
        if let Some(b) = body {
            rb = rb.json(&b);
        }

        let response = rb.send().await.map_err(|e| AppError::NetworkError {
            message: e.to_string(),
        })?;

        let status = response.status();
        if !status.is_success() {
            return Err(Self::handle_error_response(response).await);
        }

        let wrapper = response
            .json::<HttpResponseApi<O>>() // HttpResponseApi suele mapear a PaginatedResponse
            .await
            .map_err(|e| AppError::ParseError {
                message: format!("Error decoding pagination: {}", e),
            })?;

        Ok(wrapper.data)
    }

    pub async fn do_get<O: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<O, AppError> {
        // 1. Reutilizamos la URL base y el cliente existente
        let url = format!("{}{}", self.get_base_url_internal(), endpoint);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::NetworkError {
                message: e.to_string(),
            })?;

        // 2. Extraemos el status
        let status = response.status();

        // 3. Si falla, usamos nuestro manejador centralizado de errores
        if !status.is_success() {
            return Err(Self::handle_error_response(response).await);
        }

        // 4. Si tiene éxito, parseamos el JSON directamente al tipo O
        // Nota: Aquí asumo que do_get recibe el objeto directo,
        // si el backend lo envuelve en un "data", usa HttpResponseObject<O>
        let data = response
            .json::<O>()
            .await
            .map_err(|e| AppError::ParseError {
                message: format!("Error al parsear respuesta: {}", e),
            })?;

        Ok(data)
    }
}

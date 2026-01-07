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
        crate::config::config::get_base_url()
    }

    pub async fn request_one<I, O>(
        &self,
        method: reqwest::Method,
        endpoint: &str, // Ej: "/users" o "/products/303"
        body: Option<I>,
        query_params: Option<&[(&str, &str)]>, // Query params opcionales
    ) -> Result<O, AppError>
    where
        I: serde::Serialize,
        O: serde::de::DeserializeOwned,
    {
        // 1. Construcción de la URL base + controlador
        let base = self.get_base_url_internal();
        let url = format!("{}{}", base, endpoint);

        // 2. Iniciamos el constructor de la petición
        let mut rb = self.client.request(method.clone(), &url);

        // 3. Adjuntamos Query Params si existen (ej: ?page=1&limit=10)
        if let Some(params) = query_params {
            rb = rb.query(params);
        }

        // 4. Adjuntamos Body (Solo si no es GET/HEAD)
        if method != reqwest::Method::GET && method != reqwest::Method::HEAD {
            if let Some(b) = body {
                rb = rb.json(&b);
            }
        }

        // 5. Inyectar Token Automático (desde el storage global si lo tienes)
        // rb = rb.header("Authorization", format!("Bearer {}", self.get_token()));

        let response = rb.send().await.map_err(|e| AppError::NetworkError {
            message: e.to_string(),
        })?;

        // 6. Parseo del objeto (esperando tu estructura con "success", "data", etc.)
        let wrapper = response
            .json::<HttpResponseObject<O>>()
            .await
            .map_err(|e| AppError::NetworkError {
                message: format!("Error decoding: {}", e),
            })?;

        Ok(wrapper.data)
    }

    pub async fn request_paginated<I, O>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<I>,
    ) -> Result<PaginatedResponse<O>, AppError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let base = get_base_url();
        let url = format!("{}{}", base, endpoint);
        let response = self
            .client
            .request(method, &url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::NetworkError {
                message: e.to_string(),
            })?;

        let wrapper =
            response
                .json::<HttpResponseApi<O>>()
                .await
                .map_err(|e| AppError::NetworkError {
                    message: format!("Error parseando paginación: {}", e),
                })?;

        Ok(wrapper.data)
    }
}

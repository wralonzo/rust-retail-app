use super::AuthBridge;
use crate::use_cases::get_google_config_use_case::GetGoogleConfigUseCase;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl AuthBridge {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> AuthBridge {
        Self {
            use_case: Arc::new(Self::build_use_case()),
            google_use_case: Arc::new(GetGoogleConfigUseCase::new()),
        }
    }

    #[wasm_bindgen(js_name = login)]
    pub async fn login_wasm(&self, email: String, password: String) -> Result<JsValue, JsValue> {
        let user = self
            .internal_login(email, password)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(serde_wasm_bindgen::to_value(&user).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    #[wasm_bindgen(js_name = getUsers)]
    pub async fn get_users_wasm(&self, page: u32) -> Result<JsValue, JsValue> {
        let user_page = self
            .internal_fetch_users(page)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(serde_wasm_bindgen::to_value(&user_page)
            .map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    #[wasm_bindgen(js_name = getIdGoogleClient)]
    pub async fn get_id_google_client_wasm(&self) -> Result<JsValue, JsValue> {
        let result = self.internal_get_google_client().await.map_err(|e| {
            serde_wasm_bindgen::to_value(&e).unwrap_or_else(|_| JsValue::from_str("Unknown Error"))
        })?;

        // Convertimos el resultado exitoso a JsValue
        Ok(serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }
}

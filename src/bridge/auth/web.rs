use super::AuthBridge;
use wasm_bindgen::prelude::*;
use crate::bridge::main_bridge::AppContainer;
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND: &'static str = r#"
import { User } from "./User";
import { GoogleClientId } from "./GoogleClientId";
import { PaginatedResponse } from "./PaginatedResponse";

// Esto se fusionará con la clase generada por wasm-bindgen
export interface AuthBridge {
    login(email: string, password: string): Promise<User>;
    getUsers(page: number): Promise<PaginatedResponse<User>>;
    getIdGoogleClient(): Promise<GoogleClientId>;
    loginGoogle(app_google_id: string): Promise<User>;
}
"#;

#[wasm_bindgen]
impl AuthBridge {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> AuthBridge {
        let container = AppContainer::get_instance();
        Self::new_internal(container)
    }

    #[wasm_bindgen(js_name = login, skip_typescript)]
    pub async fn login_wasm(&self, email: String, password: String) -> Result<JsValue, JsValue> {
        let user = self
            .internal_login(email, password)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(serde_wasm_bindgen::to_value(&user).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    #[wasm_bindgen(js_name = getUsers, skip_typescript)]
    pub async fn get_users_wasm(&self, page: u32) -> Result<JsValue, JsValue> {
        let user_page = self
            .internal_fetch_users(page)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(serde_wasm_bindgen::to_value(&user_page)
            .map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    #[wasm_bindgen(js_name = getIdGoogleClient, skip_typescript)]
    pub async fn get_id_google_client_wasm(&self) -> Result<JsValue, JsValue> {
        let result = self.internal_get_google_client().await.map_err(|e| {
            serde_wasm_bindgen::to_value(&e).unwrap_or_else(|_| JsValue::from_str("Unknown Error"))
        })?;

        // Convertimos el resultado exitoso a JsValue
        Ok(serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    #[wasm_bindgen(js_name = iniSession)]
    pub async fn init_session(&self) -> Result<bool, JsValue> {
        // 1. Obtenemos el booleano
        let is_initialized = self.login_use_case
            .init_session()
            .await;

        // 2. Retornamos Ok con el valor.
        // (Si init_session devolviera un Result, usarías map_err aquí)
        Ok(is_initialized)
    }

    #[wasm_bindgen(js_name = logout)]
    pub async fn logout(&self) -> Result<(), JsValue> {
        self.login_use_case
            .execute_logout()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = loginGoogle, skip_typescript)]
    pub async fn login_google_wasm(&self, app_google_id: String) -> Result<JsValue, JsValue> {
        let user = self
            .internal_login_google(app_google_id)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(serde_wasm_bindgen::to_value(&user).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    #[wasm_bindgen(js_name = hydrate)]
    pub async fn hydrate(&self) -> Result<(), JsValue> {
        let container = AppContainer::get_instance();

        container.hydrate_from_db()
            .await
            // Convertimos el posible AppError a un String y luego a JsValue
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

}

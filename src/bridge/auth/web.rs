use super::AuthBridge;
use crate::bridge::main_bridge::AppContainer;
use crate::domain::models::errors::AppError;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND: &'static str = r#"
import { User } from "./User";
import { Profile } from "./Profile";
import { UserAuth } from "./UserAuth";
import { Employee } from "./Employee";
import { GoogleClientId } from "./GoogleClientId";
import { PaginatedResponse } from "./PaginatedResponse";

// Esto se fusionará con la clase generada por wasm-bindgen
export interface AuthBridge {
    login(email: string, password: string): Promise<User>;
    getUsers(page: number): Promise<PaginatedResponse<User>>;
    getIdGoogleClient(): Promise<GoogleClientId>;
    loginGoogle(app_google_id: string): Promise<User>;
    getUserLocal(): Promise<User | null>; // <--- Añadido
}
"#;

#[wasm_bindgen]
impl AuthBridge {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> Result<AuthBridge, JsValue> {
        // 1. IMPORTANTE: Esto permite ver el mensaje real del error en la consola de Chrome
        #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();

        // 2. Intentar obtener la instancia.
        // Si falla aquí, el mensaje de error de AppContainer aparecerá en la consola.
        let container = AppContainer::get_instance();

        Ok(Self::new_internal(container))
    }

    fn map_to_js(e: AppError) -> JsValue {
        serde_wasm_bindgen::to_value(&e).unwrap_or_else(|_| JsValue::from_str("Auth Error"))
    }

    #[wasm_bindgen(js_name = login, skip_typescript)]
    pub async fn login_wasm(&self, email: String, password: String) -> Result<JsValue, JsValue> {
        let user = self
            .internal_login(email, password)
            .await
            .map_err(Self::map_to_js)?;
        Ok(serde_wasm_bindgen::to_value(&user).unwrap())
    }

    #[wasm_bindgen(js_name = getIdGoogleClient, skip_typescript)]
    pub async fn get_id_google_client_wasm(&self) -> Result<JsValue, JsValue> {
        let result = self
            .internal_get_google_client()
            .await
            .map_err(Self::map_to_js)?;
        Ok(serde_wasm_bindgen::to_value(&result).unwrap())
    }

    #[wasm_bindgen(js_name = iniSession)]
    pub async fn init_session(&self) -> Result<bool, JsValue> {
        // 1. Obtenemos el booleano
        let is_initialized = self.login_use_case.init_session().await;

        // 2. Retornamos Ok con el valor.
        // (Si init_session devolviera un Result, usarías map_err aquí)
        Ok(is_initialized)
    }

    #[wasm_bindgen(js_name = loginGoogle, skip_typescript)]
    pub async fn login_google_wasm(&self, app_google_id: String) -> Result<JsValue, JsValue> {
        let user = self
            .internal_login_google(app_google_id)
            .await
            .map_err(Self::map_to_js)?;
        Ok(serde_wasm_bindgen::to_value(&user).unwrap())
    }

    #[wasm_bindgen(js_name = hydrate)]
    pub async fn hydrate(&self) -> Result<(), JsValue> {
        let container = AppContainer::get_instance();

        container
            .hydrate_session()
            .await
            // Convertimos el posible AppError a un String y luego a JsValue
            .map_err(Self::map_to_js)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = logout)]
    pub async fn logout(&self) {
        AppContainer::get_instance().logout().await;
    }

    #[wasm_bindgen(js_name = getUserLocal, skip_typescript)]
    pub async fn get_user_local_wasm(&self) -> Result<JsValue, JsValue> {
        // Llamamos a la lógica interna que devuelve Result<Option<User>, AppError>
        let container = AppContainer::get_instance();
        // Llamamos a la función que ya implementaste en AppContainer
        let user_option = container.get_user_local().await.map_err(Self::map_to_js)?;

        // Convertimos Option<User> -> User (Object) o null (JsValue::NULL)
        Ok(serde_wasm_bindgen::to_value(&user_option).unwrap_or(JsValue::NULL))
    }

    #[wasm_bindgen(js_name = getConfig, skip_typescript)]
    pub fn get_config_wasm(&self) -> String {
        let container = AppContainer::get_instance();
        // ✅ Leemos del lock, clonamos el contenido y manejamos el posible error de lock
        container
            .http_client
            .base_url
            .read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| "Error: Lock poisoned".to_string())
    }
}

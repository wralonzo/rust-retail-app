// use crate::domain::models::errors::AppError;
// use crate::domain::models::login_request::LoginRequest;
// use crate::domain::models::user::{User, UserPage};
// use crate::use_cases::auth::LoginUseCase;
// use crate::use_cases::get_google_config_use_case::GetGoogleConfigUseCase;
// use std::sync::Arc;

// // Importación condicional para evitar errores de scope
// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::*;

// // 1. DEFINICIÓN DEL STRUCT PROTEGIDA
// // uniffi::Object solo se aplica en nativo, wasm_bindgen en web.
// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Object))]
// pub struct AuthBridge {
//     use_case: Arc<LoginUseCase>,
//     google_use_case: Arc<GetGoogleConfigUseCase>,
// }

// // 2. LÓGICA COMPARTIDA (CORE)
// impl AuthBridge {
//     fn build_use_case() -> LoginUseCase {
//         let api_service = crate::infrastructure::api_service::ApiService::new();
//         let auth_repo = crate::infrastructure::auth_repository::AuthRepository::new(api_service);
//         LoginUseCase::new(auth_repo)
//     }

//     async fn internal_login(&self, user: String, pass: String) -> Result<User, AppError> {
//         let request = LoginRequest {
//             username: user,
//             password: pass,
//         };
//         self.use_case.execute(request).await
//     }

//     async fn internal_fetch_users(&self, page: u32) -> Result<UserPage, AppError> {
//         let raw = self.use_case.get_all_users(page).await?;
//         Ok(UserPage {
//             content: raw.content,
//             total_elements: raw.total_elements,
//             total_pages: raw.total_pages,
//             current_page: raw.number,
//         })
//     }

//     async fn internal_get_google_client(
//         &self,
//     ) -> Result<HttpResponseApiFindOne<GoogleClientId>, String> {
//         self.google_use_case.execute().await
//     }
// }

// // 3. EXPORTACIÓN PARA MOBILE (UniFFI)
// #[cfg(not(target_arch = "wasm32"))]
// #[uniffi::export]
// impl AuthBridge {
//     #[uniffi::constructor]
//     pub fn new() -> Arc<Self> {
//         Arc::new(Self {
//             use_case: Arc::new(Self::build_use_case()),
//             google_use_case: Arc::new(GetGoogleConfigUseCase::new()), // <--- Faltaba inicializarlo
//         })
//     }
//     pub async fn login(&self, email: String, password: String) -> Result<User, AppError> {
//         self.internal_login(email, password).await
//     }

//     pub async fn fetch_users(&self, page: u32) -> Result<UserPage, AppError> {
//         self.internal_fetch_users(page).await
//     }

//     pub async fn get_id_google_client(
//         &self,
//     ) -> Result<HttpResponseApiFindOne<GoogleClientId>, String> {
//         self.internal_get_google_client().await
//     }
// }

// // 4. EXPORTACIÓN PARA WEB (WASM / Angular)
// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen]
// impl AuthBridge {
//     #[wasm_bindgen(constructor)]
//     pub fn new_wasm() -> AuthBridge {
//         Self {
//             use_case: Arc::new(Self::build_use_case()),
//             google_use_case: Arc::new(GetGoogleConfigUseCase::new()),
//         }
//     }

//     #[wasm_bindgen(js_name = login)]
//     pub async fn login_wasm(&self, email: String, password: String) -> Result<JsValue, JsValue> {
//         let user = self
//             .internal_login(email, password)
//             .await
//             .map_err(|e| JsValue::from_str(&e.to_string()))?;

//         Ok(serde_wasm_bindgen::to_value(&user).map_err(|e| JsValue::from_str(&e.to_string()))?)
//     }

//     #[wasm_bindgen(js_name = getUsers)]
//     pub async fn get_users_wasm(&self, page: u32) -> Result<JsValue, JsValue> {
//         let user_page = self
//             .internal_fetch_users(page)
//             .await
//             .map_err(|e| JsValue::from_str(&e.to_string()))?;

//         Ok(serde_wasm_bindgen::to_value(&user_page)
//             .map_err(|e| JsValue::from_str(&e.to_string()))?)
//     }

//     #[wasm_bindgen(js_name = getIdGoogleClient)]
//     pub async fn get_id_google_client_wasm(&self) -> Result<JsValue, JsValue> {
//         let result = self
//             .internal_get_google_client()
//             .await
//             .map_err(|e| JsValue::from_str(&e))?;

//         Ok(serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))?)
//     }
// }

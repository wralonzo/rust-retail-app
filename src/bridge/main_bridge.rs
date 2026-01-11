use crate::domain::models::errors::AppError;
use crate::domain::models::user::User;
use crate::domain::storage::storage::SecureStorage;
use crate::infrastructure::http_client_rust::HttpClientRust;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
use crate::domain::storage::web_storage::WebStorage;

#[cfg(not(target_arch = "wasm32"))]
use crate::domain::storage::sqlite_storage::SqliteStorage;
use crate::infrastructure::http_repository::HttpRepository;

// Esta instancia será única para toda la vida de la aplicación Web/Mobile
static INSTANCE: OnceCell<AppContainer> = OnceCell::new();
pub struct AppContainer {
    pub storage: Arc<dyn SecureStorage>,
    pub http_client: Arc<HttpClientRust>,
    pub http_repository: Arc<HttpRepository>,
}

#[wasm_bindgen(js_name = initCoreConfig)]
pub fn init_core_config_wasm(url: String) {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    // Log para depuración (opcional pero recomendado)
    // web_sys::console::log_1(&format!("Rust: Inicializando con URL {}", url).into());

    AppContainer::init(url);
}

impl AppContainer {
    pub fn init(base_url: String) {
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "application/json".to_string());

        // 1. Inicializar Cliente HTTP
        let http_client = Arc::new(HttpClientRust::new(base_url, headers));
        let http_repository = Arc::new(HttpRepository::new(Arc::clone(&http_client)));

        // 2. Inicializar Storage según plataforma
        let storage: Arc<dyn SecureStorage> = {
            #[cfg(target_arch = "wasm32")]
            {
                Arc::new(WebStorage::new())
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                Arc::new(SqliteStorage::new("retail_shop.db"))
            }
        };

        let container = Self {
            http_client,
            storage,
            http_repository,
        };

        // Guardar en la instancia global
        let _ = INSTANCE.set(container);
    }

    /// Obtiene la instancia global.
    pub fn get_instance() -> &'static Self {
        INSTANCE.get().expect(
            "AppContainer debe ser inicializado con AppContainer::init(url) antes de usarse",
        )
    }

    pub async fn hydrate_session(&self) -> Result<(), AppError> {
        // 1. Buscamos el token en la tabla de preferencias (Storage Seguro)
        let token = match self.storage.get_preference("auth_token").await {
            Ok(Some(t)) => t,
            _ => {
                log::info!("ℹ️ No se encontró token en el storage");
                return Ok(());
            }
        };

        // 2. Inyectamos el token en el cliente HTTP de inmediato
        self.http_client.set_token(token);
        log::info!("✅ Token inyectado en el cliente HTTP");

        // 3. Intentamos recuperar los datos del usuario para log o estado interno
        if let Ok(Some(user)) = self.storage.get_session().await {
            log::info!("👤 Sesión hidratada para: {}", user.username);
        }

        Ok(())
    }

    pub async fn get_user_local(&self) -> Result<Option<User>, AppError> {
        // Intentamos obtener la sesión
        match self.storage.get_session().await {
            Ok(Some(user)) => {
                log::info!("✅ Sesión recuperada de DB: {}", user.username);
                Ok(Some(user))
            }
            Ok(None) => {
                log::info!("ℹ️ No se encontró sesión previa activa");
                Ok(None)
            }
            Err(e) => {
                log::error!("❌ Error al consultar la DB: {:?}", e);
                Err(AppError::DatabaseError {
                    message: e.to_string(),
                })
            }
        }
    }

    pub async fn logout(&self) {
        // Borrar objeto usuario
        let _ = self.storage.delete_session().await;
        // Borrar token de preferencia
        let _ = self.storage.delete_preference("auth_token").await;
        // Limpiar RAM
        self.http_client.clear_token();
    }
}

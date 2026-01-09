use once_cell::sync::Lazy;
use std::sync::Arc;
use crate::domain::models::errors::AppError;
use crate::domain::storage::storage::SecureStorage;
use crate::infrastructure::api_service::ApiService;

#[cfg(target_arch = "wasm32")]
use crate::domain::storage::web_storage::WebStorage;

#[cfg(not(target_arch = "wasm32"))]
use crate::domain::storage::sqlite_storage::SqliteStorage;

// Esta instancia será única para toda la vida de la aplicación Web/Mobile
static SHARED_CONTAINER: Lazy<AppContainer> = Lazy::new(|| AppContainer::new_internal());
pub struct AppContainer {
    pub api_service: Arc<ApiService>,
    pub storage: Arc<dyn SecureStorage>,
}

impl AppContainer {
    pub fn get_instance() -> &'static Self {
        &SHARED_CONTAINER
    }

    // Cambiamos el nombre del constructor real a uno privado o interno
    fn new_internal() -> Self {
        let api_service = Arc::new(ApiService::new());
        let storage: Arc<dyn SecureStorage> = {
            #[cfg(target_arch = "wasm32")]
            { Arc::new(WebStorage::new()) }
            #[cfg(not(target_arch = "wasm32"))]
            { Arc::new(SqliteStorage::new("retail_shop.db")) }
        };

        let container = Self { api_service, storage };

        // Intentar recuperar la sesión de forma asíncrona "disparada"
        // O mejor aún, llama a un método de hidratación después de crear el bridge
        container
    }

    // En main_bridge.rs
    pub async fn hydrate_from_db(&self) -> Result<(), AppError> {
        if let Ok(Some(user)) = self.storage.get_session().await {
            // Validamos el token aquí
            if let Some(token) = user.token {
                log::info!("✅ Sesión recuperada para: {}", user.username);
                self.api_service.set_token(token);
                Ok(())
            } else {
                log::warn!("⚠️ Sesión encontrada pero el token es nulo");
                Ok(())
            }
        } else {
            log::info!("ℹ️ No hay sesión previa");
            Ok(())
        }
    }
}
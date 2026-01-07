use rust_retail::config::config::{init_core_config};
use rust_retail::infrastructure::api_service::ApiService;

// 1. Setup compartido
// Lo movemos fuera del módulo para que sea accesible fácilmente
#[cfg(test)]
fn setup_test_config(url: &str) {
    // Usamos el resultado para evitar pánicos si se inicializa varias veces
    let _ = init_core_config(url.to_string());
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_api_service_uses_global_url() {
        // 2. Definimos la URL esperada
        let expected_url = "http://localhost:8080/app/api";

        // 3. Inicializamos
        setup_test_config(expected_url);

        // 4. Creamos el servicio
        let api = ApiService::new();

        // 5. Obtenemos la URL a través del helper del servicio
        let result = api.get_base_url_internal();

        // 6. Verificación
        assert_eq!(
            result, expected_url,
            "El ApiService no está retornando la URL global configurada"
        );
    }
}

use rust_retail::infrastructure::api_service::ApiService;

// 1. Setup compartido
// Lo movemos fuera del módulo para que sea accesible fácilmente

#[cfg(test)]
mod integration_tests {
    use wiremock::MockServer;
    use super::*;

    #[tokio::test]
    async fn test_api_service_uses_global_url() {
        // 2. Definimos la URL esperada
        let expected_url = "http://localhost:8080/app/api";
        let server = MockServer::start().await;

        // 3. Inicializamos
        let base_url = server.uri();

        // 4. Creamos el servicio
        let api = ApiService::new(base_url);

        // 5. Obtenemos la URL a través del helper del servicio
        let result = api.get_base_url_internal();

        // 6. Verificación
       
    }
}

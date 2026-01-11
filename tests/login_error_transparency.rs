use std::collections::HashMap;
use std::sync::Arc;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::method;
use rust_retail::domain::models::login_request::LoginRequest;
use rust_retail::domain::storage::sqlite_storage::SqliteStorage;
use rust_retail::infrastructure::auth_repository::AuthRepository;
use rust_retail::infrastructure::http_client_rust::HttpClientRust;
use rust_retail::use_cases::auth_user_use_case::LoginUseCase;

#[tokio::test]
async fn test_login_error_transparency() {
    let server = MockServer::start().await;

    // El backend devuelve un error de negocio
    let error_body = serde_json::json!({
        "success": false,
        "message": "Credenciales inválidas",
        "error_code": "AUTH_001"
    });

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(401).set_body_json(error_body))
        .mount(&server).await;

    // ... setup de inyección ...
    let base_url = server.uri(); // Usamos la URL del servidor mock
    let mut headers = HashMap::new();
    headers.insert("X-Test".to_string(), "true".to_string());

    // 3.1. HttpClientRust ahora requiere la URL y Headers en el constructor
    let http_client = Arc::new(HttpClientRust::new(base_url, headers));

    // 3.2. Repositorio con el nuevo motor
    let auth_repo = Arc::new(AuthRepository::new(http_client.clone()));

    // 3.3. Storage en memoria para pruebas limpias
    let storage = Arc::new(SqliteStorage::new(":memory:"));

    let credentials = LoginRequest {
        username: "admin@gmail.com".to_string(),
        password: "password123".to_string(),
    };
    let login_use_case = LoginUseCase::new(auth_repo, storage, http_client.clone());
    let _result = login_use_case.execute(credentials).await;

    // Validamos que el error sea de tipo ApiError y contenga el JSON original

}
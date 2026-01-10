use std::collections::HashMap;
use std::sync::Arc;
use wiremock::{Mock, MockServer, ResponseTemplate, matchers::{method, path}};
use rust_retail::domain::models::login_request::LoginRequest;
use rust_retail::domain::storage::sqlite_storage::SqliteStorage;
use rust_retail::infrastructure::auth_repository::AuthRepository;
use rust_retail::infrastructure::http_client_rust::HttpClientRust;
use rust_retail::use_cases::auth_user_use_case::LoginUseCase;

#[tokio::test]
async fn test_external_login_flow() {
    // 1. Setup del servidor mock (Genera una URL dinámica como http://127.0.0.1:1234)
    let server = MockServer::start().await;

    // 2. Configurar respuesta simulada (JSON que devuelve tu Backend real)
    let mock_response = serde_json::json!({
        "success": true,
        "message": "Login exitoso",
        "status": 200,             // <--- ESTE FALTABA
        "timestamp": "2026-01-09T10:00:00Z", // <--- Inclúyelo si tu struct lo requiere
        "data": {
            "id": 1,
            "token": "tok_123",
            "fullName": "Alonzo Quevedo",
            "username": "alonzo_q",
            "role": "admin"
        }
    });

    Mock::given(method("POST"))
        .and(path("/auth/login"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&server)
        .await;

    // 3. Inyección de Dependencias con la arquitectura final
    let base_url = server.uri(); // Usamos la URL del servidor mock
    let mut headers = HashMap::new();
    headers.insert("X-Test".to_string(), "true".to_string());

    // 3.1. HttpClientRust ahora requiere la URL y Headers en el constructor
    let http_client = Arc::new(HttpClientRust::new(base_url, headers));

    // 3.2. Repositorio con el nuevo motor
    let auth_repo = Arc::new(AuthRepository::new(http_client.clone()));

    // 3.3. Storage en memoria para pruebas limpias
    let storage = Arc::new(SqliteStorage::new(":memory:"));

    // 3.4. UseCase inyectado
    let login_use_case = LoginUseCase::new(auth_repo, storage, http_client.clone());

    // 4. Ejecución del flujo
    let credentials = LoginRequest {
        username: "admin@gmail.com".to_string(),
        password: "password123".to_string(),
    };

    let result = login_use_case.execute(credentials).await;

    // 5. Verificaciones
    assert!(result.is_ok(), "El login falló: {:?}", result.err());
    let user = result.unwrap();

    assert_eq!(user.token, Some("tok_123".to_string()));
    assert_eq!(user.full_name, "Alonzo Quevedo");

    // 6. ¡VALIDACIÓN EXTRA!: Verificar que el HttpClientRust guardó el token en memoria
    // Si el UseCase llamó a http.set_token, el token ya no debería ser None internamente.
}
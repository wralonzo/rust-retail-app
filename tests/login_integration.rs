use rust_retail::config::config::init_core_config;
use rust_retail::domain::models::login_request::LoginRequest;
use rust_retail::infrastructure::api_service::ApiService;
use rust_retail::infrastructure::auth_repository::AuthRepository;
use rust_retail::use_cases::auth::LoginUseCase; // Importante para evitar el pánico

#[tokio::test]
async fn test_external_login_flow() {
    // 1. Setup del servidor mock
    let server = wiremock::MockServer::start().await;

    // 2. ¡CRUCIAL!: Inicializar el Core con la URL del servidor mock
    // Esto evita el error "API_URL no inicializada"
    init_core_config(server.uri());

    // 3. Configurar respuesta simulada
    let mock_response = serde_json::json!({
        "success": true,
        "message": "Login exitoso",
        "data": {
            "id": 1,
            "user": "alonzo123",
            "name": "Alonzo",
            "role": "admin",
            "token": "tok_123",
            "fullName": "Alonzo Quevedo", // Será mapeado a full_name por serde(rename)
            "username": "alonzo_q",
            "phone": "12345678",
            "address": "Calle 123",
            "avatar": null,
            "password": null,
            "createdAt": "2026-01-07T20:30:00Z",
            "updateAt": null,
            "deletedAt": null
        },
        "status": 200,
        "timestamp": "2026-01-07T20:30:00Z"
    });

    wiremock::Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/auth/login"))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&server)
        .await;

    // 4. Inyección de Dependencias
    let api_service = ApiService::new();
    let auth_repo = AuthRepository::new(api_service);
    let login_use_case = LoginUseCase::new(auth_repo);

    // 5. Ejecutar la lógica de negocio
    let credentials = LoginRequest {
        username: "admin@gmail.com".to_string(),
        password: "n3z00N@beQ7(".to_string(),
    };

    let response = login_use_case.execute(credentials).await;

    // 6. Validar resultados
    assert!(response.is_ok(), "El login falló: {:?}", response.err());

    let user = response.unwrap();

    // Verificaciones finales
    assert_eq!(user.id, 1);
    assert_eq!(user.token, "tok_123");
    assert_eq!(user.full_name, "Alonzo Quevedo");
}

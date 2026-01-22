use rust_retail::domain::models::login_request::LoginRequest;
use rust_retail::domain::storage::storage::SecureStorage;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

mod common;

#[tokio::test]
async fn test_successful_login_flow() {
    let (server, login_use_case, storage) = common::setup_use_case().await;

    let token_value = "tok_123_secure_value";

    let mock_response = serde_json::json!({
        "success": true,
        "message": "Login exitoso",
        "status": 200,
        "timestamp": "2026-01-09T10:00:00Z",
        "data": {
            "profile": {
                "id": 1,
                "username": "smisssth_staff",
                "fullName": "Carter sDoe Smith",
                "provide": "LOCAL",
                "passwordInit": null,
                "avatar": "https://cdn.example.com/avatars/jdoe.png",
                "address": "7ma Avenida, Zona 10",
                "phone": "+502 5555-1234",
                "email": "smith_ssssstaff@example.com",
                "birthDate": "1992-05-15"
            },
            "user": {
                "id": 1,
                "enabled": true,
                "provider": "LOCAL",
                "roles": ["ROLE_ADMIN"],
                "token": token_value
            },
            "employee": {
                "id": 1,
                "warehouseId": 2,
                "positionName": "Administrador",
                "positionId": 1
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/auth/login"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&server)
        .await;

    let credentials = LoginRequest {
        username: "admin@gmail.com".to_string(),
        password: "password123".to_string(),
    };

    let result = login_use_case.execute(credentials).await;

    assert!(
        result.is_ok(),
        "Login failed unexpectedly: {:?}",
        result.err()
    );
    let user = result.unwrap();

    // Verify mapping
    assert_eq!(user.user.token, Some(token_value.to_string()));
    assert_eq!(user.profile.full_name, "Carter sDoe Smith");
    assert_eq!(user.profile.username, "smisssth_staff");

    // Verify Persistence (Session & Token)
    let stored_session = storage.get_session().await;
    assert!(stored_session.is_ok());
    assert!(
        stored_session.unwrap().is_some(),
        "Session should be stored in DB"
    );

    let stored_token = storage.get_preference("auth_token").await;
    assert!(stored_token.is_ok());
    assert_eq!(
        stored_token.unwrap(),
        Some(token_value.to_string()),
        "Token should be stored as preference"
    );

    // Check if HTTP client has the token set (indirectly via use case logic)
    // Assuming implementation details, but for integration test relies on behavior.
    assert!(
        login_use_case.init_session().await,
        "init_session should recognize the stored token"
    );
}

use rust_retail::domain::models::login_request::LoginRequest;
use rust_retail::domain::storage::storage::SecureStorage;
use wiremock::{matchers::method, Mock, ResponseTemplate};

mod common;

#[tokio::test]
async fn test_init_session_flow() {
    let (_server, login_use_case, storage) = common::setup_use_case().await;

    // 1. Initially no session
    assert!(!login_use_case.init_session().await);

    // 2. Manually inject token and session into storage (simulate previous login)
    let _ = storage
        .save_preference("auth_token", "restored_token_123")
        .await;

    // 3. init_session should now succeed
    assert!(login_use_case.init_session().await);
}

#[tokio::test]
async fn test_logout_clears_data() {
    let (server, login_use_case, storage) = common::setup_use_case().await;

    // Login logic first to populate data
    let token_value = "token_to_delete";
    let mock_response = serde_json::json!({
        "success": true, "message": "OK", "status": 200, "timestamp": "",
        "data": {
            "profile": { "id": 1, "username": "u", "fullName": "n", "provider": "L", "passwordInit": null, "avatar": null, "address": null, "phone": null, "email": null, "birthDate": null },
            "user": { "id": 1, "enabled": true, "provider": "L", "roles": [], "token": token_value },
            "employee": null
        }
    });

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&server)
        .await;
    let _ = login_use_case
        .execute(LoginRequest {
            username: "a@b.c".into(),
            password: "p".into(),
        })
        .await;

    // Verify stored
    assert!(storage
        .get_preference("auth_token")
        .await
        .unwrap()
        .is_some());

    // Logout
    let logout_res = login_use_case.execute_logout().await;
    assert!(logout_res.is_ok());

    // Verify cleared
    assert!(storage
        .get_preference("auth_token")
        .await
        .unwrap()
        .is_none());
    assert!(storage.get_session().await.unwrap().is_none());
}

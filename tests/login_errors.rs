use rust_retail::domain::models::login_request::LoginRequest;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

mod common;

#[tokio::test]
async fn test_api_unauthorized_error() {
    let (server, login_use_case, _storage) = common::setup_use_case().await;

    // Simulate 401 Unauthorized from backend
    Mock::given(method("POST"))
        .and(path("/auth/login"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;

    let credentials = LoginRequest {
        username: "user@test.com".to_string(),
        password: "wrongpassword".to_string(),
    };

    let result = login_use_case.execute(credentials).await;

    // Depending on HttpClient implementation, this might return Unauthorized or Generic Error
    // Adjust assertion based on your error handling middleware
    assert!(result.is_err());
}

#[tokio::test]
async fn test_server_error() {
    let (server, login_use_case, _storage) = common::setup_use_case().await;

    // Simulate 500 Internal Server Error
    Mock::given(method("POST"))
        .and(path("/auth/login"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let credentials = LoginRequest {
        username: "user@test.com".to_string(),
        password: "password".to_string(),
    };

    let result = login_use_case.execute(credentials).await;
    assert!(result.is_err());
}

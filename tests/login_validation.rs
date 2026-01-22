use rust_retail::domain::models::errors::AppError;
use rust_retail::domain::models::login_request::LoginRequest;

mod common;

#[tokio::test]
async fn test_validation_errors() {
    let (_server, login_use_case, _storage) = common::setup_use_case().await;

    // 1. Empty Username
    let req_empty_user = LoginRequest {
        username: "".to_string(),
        password: "password".to_string(),
    };
    let result = login_use_case.execute(req_empty_user).await;
    match result {
        Err(AppError::EmptyField { message }) => assert_eq!(message, "Usuario"),
        _ => panic!("Expected EmptyField error for username"),
    }

    // 2. Empty Password
    let req_empty_pass = LoginRequest {
        username: "valid@email.com".to_string(),
        password: "".to_string(),
    };
    let result = login_use_case.execute(req_empty_pass).await;
    match result {
        Err(AppError::EmptyField { message }) => assert_eq!(message, "Contraseña"),
        _ => panic!("Expected EmptyField error for password"),
    }
}

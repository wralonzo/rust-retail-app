use rust_retail::domain::storage::sqlite_storage::SqliteStorage;
use rust_retail::infrastructure::auth_repository::AuthRepository;
use rust_retail::infrastructure::http_client_rust::HttpClientRust;
use rust_retail::use_cases::auth_user_use_case::LoginUseCase;
use std::collections::HashMap;
use std::sync::Arc;
use wiremock::MockServer;

// Helper function to setup the use case and dependencies for testing
pub async fn setup_use_case() -> (MockServer, LoginUseCase, Arc<SqliteStorage>) {
    let server = MockServer::start().await;
    let base_url = server.uri();
    let mut headers = HashMap::new();
    headers.insert("X-Test".to_string(), "true".to_string());

    let http_client = Arc::new(HttpClientRust::new(base_url, headers));
    let auth_repo = Arc::new(AuthRepository::new(http_client.clone()));
    let storage = Arc::new(SqliteStorage::new(":memory:")); // In-memory DB
    let login_use_case = LoginUseCase::new(auth_repo, storage.clone(), http_client.clone());

    (server, login_use_case, storage)
}

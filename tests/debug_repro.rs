use rust_retail::infrastructure::http_client_rust::HttpClientRust;
use std::collections::HashMap;

#[tokio::test]
async fn test_reproduce_builder_error_with_bad_url() {
    // Simulate a base URL with a trailing newline or space
    let bad_base_url = "http://localhost:8080\n";
    let headers = HashMap::new();

    let client = HttpClientRust::new(bad_base_url.to_string(), headers);
    let result = client.get::<serde_json::Value>("/some-endpoint").await;

    match result {
        Err(e) => {
            let msg = e.to_string();
            println!("Got error: {}", msg);
            if msg.contains("builder error") {
                panic!("REPRODUCED URL ERROR: {}", msg);
            }
        }
        Ok(_) => println!("Request unexpectedly succeeded"),
    }
}

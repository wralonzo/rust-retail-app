use reqwest::Client;

pub struct ApiClient {
    _http: Client, // El guion bajo le dice a Rust: "sé que no lo uso aún"
    _base_url: String,
}

pub mod domain;
pub mod use_cases;
pub mod infrastructure;
pub mod bridge;
pub mod utils;
//pub use bridge::wasm::*;

// Esto genera el código necesario para que Kotlin/Swift se conecten
uniffi::setup_scaffolding!("rust_retail");

#[uniffi::export]
pub fn saludo(nombre: String) -> String {
    format!("Hola, {} desde Rust!", nombre)
}
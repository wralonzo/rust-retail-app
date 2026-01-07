pub mod domain;
pub mod use_cases;
pub mod infrastructure;
pub mod bridge;
pub mod config;
//pub use bridge::wasm::*;

uniffi::setup_scaffolding!();
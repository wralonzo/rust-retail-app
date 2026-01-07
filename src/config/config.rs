use std::sync::OnceLock;

// 1. El "Almacén" Privado (Solo visible en Rust)
static API_URL: OnceLock<String> = OnceLock::new();

// 2. Lógica de negocio pura (Fachada interna)
// Esta función es la que hace el trabajo real y es invisible para JS/Mobile
fn set_url(url: String) -> bool {
    API_URL.set(url).is_ok()
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = initCoreConfig)] // <--- Esto le dice a JS que se llama así
pub fn init_core_config_wasm(url: String) -> bool {
    set_url(url)
}

// Para que el compilador nativo (el que usa VS Code / Vite) no se confunda
#[cfg(not(target_arch = "wasm32"))]
pub fn init_core_config(url: String) -> bool {
    set_url(url)
}

// 5. El Getter (Usado por tu ApiService internamente)
pub fn get_base_url() -> String {
    API_URL.get()
        .cloned()
        .expect("Core Error: API_URL no inicializada. Llama a initCoreConfig() primero.")
}
#[cfg(test)]
pub fn setup_api_url() {
    // Usamos el resultado de init para que no explote si se llama dos veces
    // en el mismo hilo/proceso.
    let _ = crate::config::init_core_config("http://localhost:8080".to_string());
}
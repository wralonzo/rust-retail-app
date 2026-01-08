use serde::{Deserialize, Serialize}; // <--- Faltaba esto

#[derive(Serialize, Deserialize, uniffi::Record)] // Añadimos uniffi::Record para Mobile
pub struct GoogleClientId {
    #[serde(rename = "clientId")]
    pub client_id: String,
}

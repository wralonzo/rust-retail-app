use serde::{Deserialize, Serialize};
use ts_rs::TS; // <--- Faltaba esto

#[derive(TS, Serialize, Deserialize, uniffi::Record)] // Añadimos uniffi::Record para Mobile
#[ts(export, export_to = "GoogleClientId.ts")]
pub struct GoogleClientId {
    #[serde(rename = "clientId")]
    pub client_id: String,
}

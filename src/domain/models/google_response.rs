use serde::{Deserialize, Serialize};

use crate::domain::models::google_id::GoogleClientId;

#[derive(Serialize, Deserialize, uniffi::Record)] // Record es para structs de datos en UniFFI
pub struct GoogleResponse {
    pub data: GoogleClientId,
    pub success: bool,
    pub message: Option<String>,
}

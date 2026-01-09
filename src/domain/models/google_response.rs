use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::domain::models::google_id::GoogleClientId;

#[derive(TS, Serialize, Deserialize, uniffi::Record)]
#[ts(export, export_to = "GoogleResponse.ts")]
pub struct GoogleResponse {
    pub data: GoogleClientId,
    pub success: bool,
    pub message: Option<String>,
}

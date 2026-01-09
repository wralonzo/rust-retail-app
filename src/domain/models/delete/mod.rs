use ts_rs::TS;
use serde::Deserialize;

#[derive(TS, Deserialize)]
pub struct DeleteMessage {
    pub data: DeleteMessageInner,
}

#[derive(TS, Deserialize)]
pub struct DeleteMessageInner {
    pub message: String,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, uniffi::Record)]
pub struct Preference {
    pub id: i32,
    pub data: String,
    pub key: String,
}

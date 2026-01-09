use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::domain::models::user::User;

#[derive(TS, Debug, Serialize, Deserialize, Clone, uniffi::Record)]
#[ts(export, export_to = "ClientResponse.ts")]
pub struct ClientResponse {
    id: i32,
    email: String,
    phone: Option<String>,
    address: Option<String>,
    notes: Option<String>,
    #[serde(rename = "birthDate")]
    birth_date: Option<String>,
    #[serde(rename = "clientType")]
    clien_type: String,
    code: String,
    user: Option<User>,
}


#[derive(TS, Debug, Serialize, Deserialize, Clone, uniffi::Record)]
#[ts(export, export_to = "ClientRequest.ts")]
pub struct ClientRequest {
    name: String,
    email: String,
    phone: String,
    address: Option<String>,
    notes: Option<String>,
    #[serde(rename = "birthDate")]
    birth_date: Option<String>,
    #[serde(rename = "clientType")]
    clien_type: String,
    #[serde(rename = "flagUser")]
    flag_user: Option<bool>,
}

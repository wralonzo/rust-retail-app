// src/domain/models/user.rs
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Serialize, Deserialize, Clone, uniffi::Record)]
#[ts(export, export_to = "User.ts")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub token: Option<String>,

    #[serde(rename = "fullName")]
    pub full_name: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub password: Option<String>,

    #[serde(rename = "passwordInit")]
    pub password_init: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,

    #[serde(rename = "updateAt")]
    pub update_at: Option<String>,

    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<String>,

    pub employee: Option<Employee>,
    pub roles: Option<Vec<String>>,
    pub enabled: Option<bool>,

    #[serde(rename = "clientId")]
    pub client_id: Option<i32>,

    pub provider: Option<String>,
}

#[derive(TS, Debug, Serialize, Deserialize, Clone, uniffi::Record)]
#[ts(export, export_to = "Employee.ts")]
pub struct Employee {
    pub id: i32,

    #[serde(rename = "warehouseId")]
    pub warehouse_id: i32,

    #[serde(rename = "warehouseName")]
    pub warehouse_name: String,

    #[serde(rename = "positionId")]
    pub position_id: i32,

    #[serde(rename = "positionName")]
    pub position_nam: String,
}

// src/domain/models/responses.rs

#[derive(TS, Debug, Clone, serde::Serialize, serde::Deserialize, uniffi::Record)]
#[ts(export, export_to = "UserPage.ts")]
pub struct UserPage {
    pub content: Vec<User>,
    pub total_elements: u64,
    pub total_pages: u32,
    pub current_page: u32,
}

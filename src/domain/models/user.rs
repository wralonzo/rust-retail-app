// src/domain/models/user.rs
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Serialize, Deserialize, Clone, uniffi::Record)]
#[ts(export, export_to = "User.ts")]
pub struct User {
    pub profile: Profile,
    pub user: UserAuth,
    pub employee: Option<Employee>,
}

#[derive(TS, Debug, Serialize, Deserialize, Clone, uniffi::Record)]
#[ts(export, export_to = "Profile.ts")]
pub struct Profile {
    pub id: i32,
    pub username: String,

    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(alias = "provide")]
    pub provider: String,

    #[serde(rename = "passwordInit")]
    pub password_init: Option<String>,
    pub avatar: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,

    #[serde(rename = "birthDate")]
    pub birth_date: Option<String>,
}

#[derive(TS, Debug, Serialize, Deserialize, Clone, uniffi::Record)]
#[ts(export, export_to = "UserAuth.ts")]
pub struct UserAuth {
    pub id: i32,
    pub enabled: bool,
    pub provider: String,
    pub roles: Vec<String>,
    pub token: Option<String>,
}

#[derive(TS, Debug, Serialize, Deserialize, Clone, uniffi::Record)]
#[ts(export, export_to = "Employee.ts")]
pub struct Employee {
    pub id: i32,

    #[serde(rename = "warehouseId")]
    pub warehouse_id: i32,

    #[serde(rename = "positionName")]
    pub position_name: String,

    #[serde(rename = "positionId")]
    pub position_id: i32,
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

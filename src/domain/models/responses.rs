use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginatedResponse<T> {
    pub content: Vec<T>,

    #[serde(rename = "totalElements")]
    pub total_elements: u64, // Cambiado de usize a u64 (64 bits fijo)

    #[serde(rename = "totalPages")]
    pub total_pages: u32, // Cambiado de usize a u32 (32 bits fijo)

    pub size: u32,   // Cambiado de usize a u32
    pub number: u32, // Cambiado de usize a u32
    pub last: bool,
    pub first: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpResponseApi<T> {
    pub success: bool,
    pub message: String,
    pub data: PaginatedResponse<T>,
    pub status: u16,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpResponseObject<T> {
    pub success: bool,
    pub message: String,
    pub data: T,
    pub status: u16,
    pub timestamp: String,
}

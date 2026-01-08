pub mod storage;
#[cfg(target_arch = "wasm32")]
pub mod web_storage;

#[cfg(not(target_arch = "wasm32"))]
pub mod sqlite_storage;

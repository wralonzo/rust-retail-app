use std::sync::Arc;
use crate::bridge::main_bridge::AppContainer;
use crate::infrastructure::client::client_repository::{ClientRepository};
use crate::use_cases::client::add_client_use_case::AddClientUseCase;
use crate::use_cases::client::delete_client_use_case::DeleteClientUseCase;
use crate::use_cases::client::find_client_use_case::FindClientUseCase;
use crate::use_cases::client::find_one_client_use_case::FindOneClientUseCase;
use crate::use_cases::client::update_client_use_case::UpdateClientUseCase;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub mod mobile;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Object))]
pub struct ClientBridge {
    pub(crate) add_client_use_case: Arc<AddClientUseCase>,
    pub(crate) delete_client_use_case: Arc<DeleteClientUseCase>,
    pub(crate) update_client_use_case: Arc<UpdateClientUseCase>,
    pub(crate) find_client_use_case: Arc<FindClientUseCase>,
    pub(crate) find_one_client_use_case: Arc<FindOneClientUseCase>
}

impl ClientBridge {
    pub fn new_internal(container: &AppContainer) -> Self {
        // Aquí pasamos el MISMO api_service que recibió AuthBridge
        let repo = Arc::new(ClientRepository::new(container.api_service.clone()));

        Self {
            add_client_use_case: Arc::new(AddClientUseCase::new(repo.clone())),
            delete_client_use_case: Arc::new(DeleteClientUseCase::new(repo.clone())),
            update_client_use_case: Arc::new(UpdateClientUseCase::new(repo.clone())),
            find_client_use_case: Arc::new(FindClientUseCase::new(repo.clone())),
            find_one_client_use_case: Arc::new(FindOneClientUseCase::new(repo.clone())),
        }
    }
}
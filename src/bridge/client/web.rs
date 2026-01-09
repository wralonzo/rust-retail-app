use wasm_bindgen::prelude::*;
use super::ClientBridge;
use crate::bridge::main_bridge::AppContainer;
#[wasm_bindgen(typescript_custom_section)]
const TS_CLIENT: &'static str = r#"
import { ClientResponse } from "../models/ClientResponse";
import { ClientRequest } from "../models/ClientRequest";
import { PaginatedResponse } from "../models/PaginatedResponse";

export interface ClientBridge {
    getClients(search: string, sort: string, page: number, size: number, type: string): Promise<PaginatedResponse<ClientResponse>>;
    saveClient(req: ClientRequest): Promise<ClientResponse>;
    deleteClient(id: number): Promise<string>;
    updateClient(req: ClientRequest): Promise<ClientResponse>;
}
"#;

#[wasm_bindgen]
impl ClientBridge {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> ClientBridge {
        // Debes crear o traer el contenedor aquí
        let container = AppContainer::get_instance();
        Self::new_internal(container)
    }

    #[wasm_bindgen(js_name = getClients, skip_typescript)]
    pub async fn get_clients_wasm(&self, search: String, sort: String, page: u32, size: u32, client_type: String) -> Result<JsValue, JsValue> {
        let result = self.find_client_use_case
            .execute(search, sort, page, size, client_type)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(serde_wasm_bindgen::to_value(&result).unwrap())
    }

    #[wasm_bindgen(js_name = deleteClient, skip_typescript)]
    pub async fn delete_client_wasm(&self, id: i32) -> Result<String, JsValue> {
        self.delete_client_use_case
            .execute(id)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = saveClient, skip_typescript)]
    pub async fn save_client_wasm(&self, req: JsValue) -> Result<JsValue, JsValue> {
        let request = serde_wasm_bindgen::from_value(req)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let result = self.add_client_use_case
            .execute(request)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(serde_wasm_bindgen::to_value(&result).unwrap())
    }

    #[wasm_bindgen(js_name = getClient, skip_typescript)]
    pub async fn get_client_wasm(&self, id: i32) -> Result<JsValue, JsValue> {
        let result = self.find_one_client_use_case
            .execute(id)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(serde_wasm_bindgen::to_value(&result).unwrap())
    }

    #[wasm_bindgen(js_name = updateClient, skip_typescript)]
    pub async fn update_client_wasm(&self,id: i32, req: JsValue) -> Result<JsValue, JsValue> {
        let request = serde_wasm_bindgen::from_value(req)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let result = self.update_client_use_case
            .execute(id, request)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(serde_wasm_bindgen::to_value(&result).unwrap())
    }


}
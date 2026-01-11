use super::GenericHttpBridge;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl GenericHttpBridge {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let container = crate::bridge::main_bridge::AppContainer::get_instance();
        Self::new_internal(container)
    }

    pub async fn get(&self, path: String) -> Result<JsValue, JsValue> {
        let result = self
            .internal_get(path)
            .await
            .map_err(|e| self.serialize_error(e))?; // Usamos helper para errores

        self.serialize_success(result)
    }

    pub async fn post(&self, path: String, body: JsValue) -> Result<JsValue, JsValue> {
        let body_json: serde_json::Value =
            serde_wasm_bindgen::from_value(body).map_err(|e| JsValue::from_str(&e.to_string()))?;

        let result = self
            .internal_post(path, body_json)
            .await
            .map_err(|e| self.serialize_error(e))?;

        self.serialize_success(result)
    }

    pub async fn patch(&self, path: String, body: JsValue) -> Result<JsValue, JsValue> {
        let body_json: serde_json::Value =
            serde_wasm_bindgen::from_value(body).map_err(|e| JsValue::from_str(&e.to_string()))?;

        let result = self
            .internal_patch(path, body_json)
            .await
            .map_err(|e| self.serialize_error(e))?;

        self.serialize_success(result)
    }

    pub async fn delete(&self, path: String) -> Result<JsValue, JsValue> {
        let result = self
            .internal_delete(path)
            .await
            .map_err(|err| self.serialize_error(err))?;
        self.serialize_success(result)
    }

    // --- Helpers privados para mantener consistencia ---
    fn serialize_success<T: Serialize>(&self, data: T) -> Result<JsValue, JsValue> {
        let serializer = Serializer::new().serialize_maps_as_objects(true);
        data.serialize(&serializer)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    fn serialize_error(&self, err: crate::domain::models::errors::AppError) -> JsValue {
        // Serializa el AppError (incluyendo el ApiError con el mensaje de Spring)
        serde_wasm_bindgen::to_value(&err).unwrap_or(JsValue::from_str("Internal Bridge Error"))
    }
}

use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct JSResult {
    pub status: &'static str,
    pub output: Option<String>,
    pub error: Option<String>,
}

impl JSResult {
    pub fn output(output: String) -> JsValue {
        let result = JSResult {
            status: "ok",
            output: Some(output),
            error: None,
        };

        JsValue::from_serde(&result).unwrap()
    }

    pub fn error(error: String) -> JsValue {
        let result = JSResult {
            status: "error",
            output: None,
            error: Some(error),
        };

        JsValue::from_serde(&result).unwrap()
    }
}

use js_sys::Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

pub async fn list_devices() -> Vec<(String, String)> {
    let array: Array = listDevices().await.dyn_into().unwrap();
    array
        .iter()
        .map(|v| {
            let array: Array = v.dyn_into().unwrap();
            (
                array.at(0).as_string().unwrap(),
                array.at(1).as_string().unwrap(),
            )
        })
        .collect()
}

#[wasm_bindgen(raw_module = "/js/modules/cameras.mjs")]
extern "C" {
    async fn listDevices() -> JsValue;
}

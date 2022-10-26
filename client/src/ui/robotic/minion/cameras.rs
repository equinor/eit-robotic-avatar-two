use js_sys::{Array, Reflect};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::MediaStream;

pub async fn load_cams(left: &str, right: &str) -> (MediaStream, MediaStream) {
    let streams = loadCams(left, right).await;
    let left = Reflect::get(&streams, &JsValue::from_str("left"))
        .unwrap()
        .dyn_into()
        .unwrap();
    let right = Reflect::get(&streams, &JsValue::from_str("right"))
        .unwrap()
        .dyn_into()
        .unwrap();
    (left, right)
}

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
    async fn loadCams(left: &str, right: &str) -> JsValue;
    async fn listDevices() -> JsValue;
}

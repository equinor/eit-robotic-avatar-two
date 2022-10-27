use js_sys::{Array, Reflect};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{MediaDeviceInfo, MediaDeviceKind, MediaDevices, MediaStream};

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
    let devices_promise = media_devices().enumerate_devices().unwrap();
    let devices_value = JsFuture::from(devices_promise).await.unwrap();
    let devices_array: Array = devices_value.dyn_into().unwrap();
    let devices_info = devices_array
        .iter()
        .map(|j| -> MediaDeviceInfo { j.dyn_into().unwrap() });
    let video_info = devices_info.filter(|info| info.kind() == MediaDeviceKind::Videoinput);
    video_info
        .map(|info| (info.label(), info.device_id()))
        .collect()
}

fn media_devices() -> MediaDevices {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    navigator.media_devices().unwrap()
}

#[wasm_bindgen(raw_module = "/js/modules/cameras.mjs")]
extern "C" {
    async fn loadCams(left: &str, right: &str) -> JsValue;
}

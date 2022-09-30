use js_sys::Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{MediaStream, MediaDeviceInfo};
use weblog::console_warn;
use yew::Callback;

pub fn get_user_video(callback: Callback<MediaStream>) {
    spawn_local(async move {
        match user_video_id("test id").await.dyn_into() {
            Ok(media) => callback.emit(media),
            Err(err) => console_warn!("Fail to get user video", err),
        }
    });
}


pub fn enumerate_devices_callback(callback: Callback<Vec<MediaDeviceInfo>>) {
    spawn_local(async move {
        callback.emit(enumerate_devices().await)
    });
}

pub async fn enumerate_devices() -> Vec<MediaDeviceInfo> {
    let window =  web_sys::window().unwrap();
    let navigator = window.navigator();
    let media_devices = navigator.media_devices().unwrap();
    let devices_promise = media_devices.enumerate_devices().unwrap();
    let devices_value = JsFuture::from(devices_promise).await.unwrap();
    let device_array: Array = devices_value.dyn_into().unwrap();
    device_array.iter().map(|j|j.dyn_into().unwrap()).collect()
}

#[wasm_bindgen(module = "/js/media_selector.js")]
extern "C" {
    async fn user_video_id(id: &str) -> JsValue;
}

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    MediaDeviceInfo, MediaDeviceKind, MediaDevices, MediaStream, MediaStreamConstraints,
};

pub async fn list_devices() -> Vec<MediaDeviceInfo> {
    let devices_promise = media_devices().enumerate_devices().unwrap();
    let devices_value = JsFuture::from(devices_promise).await.unwrap();
    let device_array: Array = devices_value.dyn_into().unwrap();
    device_array.iter().map(|j| j.dyn_into().unwrap()).collect()
}

pub async fn list_video() -> Vec<MediaDeviceInfo> {
    let devices = list_devices().await;
    devices
        .into_iter()
        .filter(|info| info.kind() == MediaDeviceKind::Videoinput)
        .collect()
}

pub async fn get_user_video(id: &str) -> MediaStream {
    let video = Object::new();
    Reflect::set(&video, &"deviceId".into(), &id.into()).unwrap();

    let mut constraints = MediaStreamConstraints::new();
    constraints.video(&video);

    let media = media_devices()
        .get_user_media_with_constraints(&constraints)
        .unwrap();
    JsFuture::from(media).await.unwrap().dyn_into().unwrap()
}

fn media_devices() -> MediaDevices {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    navigator.media_devices().unwrap()
}

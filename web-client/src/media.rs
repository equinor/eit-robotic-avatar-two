use js_sys::Array;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{MediaDeviceInfo, MediaDevices, MediaStream, MediaStreamConstraints};
use yew::Callback;

pub struct MediaService {
    media_devices: MediaDevices,
}

impl MediaService {
    pub fn new() -> MediaService {
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let media_devices = navigator.media_devices().unwrap();

        MediaService { media_devices }
    }

    pub fn get_user_media_callback(&self, callback: Callback<MediaStream>) {
        let mut constraints = MediaStreamConstraints::new();
        constraints.audio(&JsValue::from(true));
        constraints.video(&JsValue::from(true));
        let media = self
            .media_devices
            .get_user_media_with_constraints(&constraints)
            .unwrap();
        spawn_local(async move {
            let stream = JsFuture::from(media).await.unwrap().dyn_into().unwrap();
            callback.emit(stream);
        });
    }

    pub fn enumerate_devices_callback(&self, callback: Callback<Vec<MediaDeviceInfo>>) {
        let devices_promise = self.media_devices.enumerate_devices().unwrap();
        spawn_local(async move {
            let devices_value = JsFuture::from(devices_promise).await.unwrap();
            let device_array: Array = devices_value.dyn_into().unwrap();
            let devices = device_array.iter().map(|j| j.dyn_into().unwrap()).collect();
            callback.emit(devices);
        });
    }
}

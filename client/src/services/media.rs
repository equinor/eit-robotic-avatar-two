use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    MediaDeviceInfo, MediaDeviceKind, MediaDevices, MediaStream, MediaStreamConstraints,
};

#[derive(Clone)]
pub struct Media {
    media_devices: MediaDevices,
}

impl Media {
    pub fn new() -> Media {
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let media_devices = navigator.media_devices().unwrap();
        Media { media_devices }
    }

    pub async fn list_devices(&self) -> Vec<MediaDeviceInfo> {
        let devices_promise = self.media_devices.enumerate_devices().unwrap();
        let devices_value = JsFuture::from(devices_promise).await.unwrap();
        let device_array: Array = devices_value.dyn_into().unwrap();
        device_array.iter().map(|j| j.dyn_into().unwrap()).collect()
    }

    pub async fn list_video(&self) -> Vec<MediaDeviceInfo> {
        let devices = self.list_devices().await;
        devices
            .into_iter()
            .filter(|info| info.kind() == MediaDeviceKind::Videoinput)
            .collect()
    }

    pub async fn get_user_video(&self, id: &str) -> MediaStream {
        let video = Object::new();
        Reflect::set(&video, &"deviceId".into(), &id.into()).unwrap();
        Reflect::set(&video, &"width".into(), &1280.into()).unwrap();
        Reflect::set(&video, &"height".into(), &720.into()).unwrap();

        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&video);

        let media = self
            .media_devices
            .get_user_media_with_constraints(&constraints)
            .unwrap();
        JsFuture::from(media).await.unwrap().dyn_into().unwrap()
    }
}

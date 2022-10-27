use futures::join;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    MediaDeviceInfo, MediaDeviceKind, MediaDevices, MediaStream, MediaStreamConstraints,
};
use weblog::console_log;

pub async fn load_cams(left: &str, right: &str) -> (MediaStream, MediaStream) {
    // log list of devices to console.
    let devices = enumerate_devices().await;
    for device in devices.iter() {
        console_log!(format!(
            "{:?}: {} id = {}",
            device.kind(),
            device.label(),
            device.device_id()
        ));
    }

    let left = get_user_media(left);
    let right = get_user_media(right);

    join!(left, right)
}

pub async fn list_devices() -> Vec<(String, String)> {
    let devices_info = enumerate_devices().await;
    let video_info = devices_info
        .iter()
        .filter(|info| info.kind() == MediaDeviceKind::Videoinput);
    video_info
        .map(|info| (info.label(), info.device_id()))
        .collect()
}

fn media_devices() -> MediaDevices {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    navigator.media_devices().unwrap()
}

async fn enumerate_devices() -> Vec<MediaDeviceInfo> {
    let devices_promise = media_devices().enumerate_devices().unwrap();
    let devices_value = JsFuture::from(devices_promise).await.unwrap();
    let device_array: Array = devices_value.dyn_into().unwrap();
    device_array.iter().map(|j| j.dyn_into().unwrap()).collect()
}

async fn get_user_media(id: &str) -> MediaStream {
    let video = Object::new();
    Reflect::set(&video, &"deviceId".into(), &id.into()).unwrap();
    Reflect::set(&video, &"width".into(), &1280.into()).unwrap();
    Reflect::set(&video, &"height".into(), &720.into()).unwrap();

    let mut constraints = MediaStreamConstraints::new();
    constraints.video(&video);

    let media = media_devices()
        .get_user_media_with_constraints(&constraints)
        .unwrap();
    JsFuture::from(media).await.unwrap().dyn_into().unwrap()
}

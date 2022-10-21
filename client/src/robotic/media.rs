use std::{cell::RefCell, rc::Rc};

use js_sys::Array;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{MediaDeviceInfo, MediaDevices, MediaStream, MediaStreamConstraints};
use yew::Callback;

pub struct MediaService {
    media_devices: MediaDevices,
    on_change: Callback<()>,
    state: Rc<RefCell<MediaState>>,
}

impl MediaService {
    pub fn new(on_change: Callback<()>) -> MediaService {
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let media_devices = navigator.media_devices().unwrap();

        MediaService {
            media_devices,
            on_change,
            state: Default::default(),
        }
    }

    pub fn state(&self) -> MediaState {
        self.state.borrow().clone()
    }

    pub fn get_media(&mut self) {
        let media_devices = self.media_devices.clone();
        let state = self.state.clone();
        let on_change = self.on_change.clone();
        spawn_local(async move {
            let stream = get_user_media(&media_devices).await;
            let devices = enumerate_devices(&media_devices).await;
            let mut state_ref = state.borrow_mut();
            state_ref.left = stream;
            state_ref.devices = devices;
            drop(state_ref);
            on_change.emit(())
        })
    }
}

async fn get_user_media(media_devices: &MediaDevices) -> MediaStream {
    let mut constraints = MediaStreamConstraints::new();
    constraints.audio(&JsValue::from(true));
    constraints.video(&JsValue::from(true));
    let media = media_devices
        .get_user_media_with_constraints(&constraints)
        .unwrap();
    JsFuture::from(media).await.unwrap().dyn_into().unwrap()
}

async fn enumerate_devices(media_devices: &MediaDevices) -> Vec<MediaDeviceInfo> {
    let devices_promise = media_devices.enumerate_devices().unwrap();
    let devices_value = JsFuture::from(devices_promise).await.unwrap();
    let device_array: Array = devices_value.dyn_into().unwrap();
    device_array.iter().map(|j| j.dyn_into().unwrap()).collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MediaState {
    pub left: MediaStream,
    pub devices: Vec<MediaDeviceInfo>,
}

impl Default for MediaState {
    fn default() -> Self {
        let media = MediaStream::new().unwrap();
        Self {
            left: media,
            devices: Default::default(),
        }
    }
}

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen_futures::spawn_local;
use web_sys::{MediaDeviceInfo, MediaStream};
use yew::Callback;

use crate::services::Media;

pub struct MediaModel {
    media: Media,
    on_change: Callback<()>,
    state: Rc<RefCell<MediaState>>,
}

impl MediaModel {
    pub fn new(on_change: Callback<()>) -> MediaModel {
        MediaModel {
            media: Media::new(),
            on_change,
            state: Default::default(),
        }
    }

    pub fn state(&self) -> MediaState {
        self.state.borrow().clone()
    }

    pub fn get_media(&mut self) {
        let media = self.media.clone();
        let state = self.state.clone();
        let on_change = self.on_change.clone();
        spawn_local(async move {
            let stream = media.get_user_video("").await;
            let devices = media.list_devices().await;
            let mut state_ref = state.borrow_mut();
            state_ref.left = stream;
            state_ref.devices = devices;
            drop(state_ref);
            on_change.emit(())
        })
    }
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

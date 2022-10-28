use std::{cell::RefCell, rc::Rc};

use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use web_sys::MediaDeviceInfo;
use yew::Callback;

use crate::services::Media;

pub struct MinionModel {
    on_change: Callback<()>,
    cam_id: (String, String),
    media: Media,
    devices: Rc<RefCell<Vec<MediaDeviceInfo>>>,
}

impl MinionModel {
    pub fn new(on_change: Callback<()>) -> MinionModel {
        let cam_id = LocalStorage::get("minion_cam_id").unwrap_or_default();
        let media = Media::new();

        let model = MinionModel {
            on_change,
            cam_id,
            media,
            devices: Rc::default(),
        };
        model.get_devices();
        model
    }

    pub fn action(&mut self, action: MinionAction) {
        match action {
            MinionAction::LeftCamChange(id) => {
                self.cam_id.0 = id;
                LocalStorage::set("minion_cam_id", self.cam_id.clone()).unwrap();
                self.on_change.emit(());
            }
            MinionAction::RightCamChange(id) => {
                self.cam_id.1 = id;
                LocalStorage::set("minion_cam_id", self.cam_id.clone()).unwrap();
                self.on_change.emit(());
            }
        }
    }

    pub fn state(&self) -> MinionState {
        MinionState {
            cam_id: self.cam_id.clone(),
            devices: self.devices.borrow().clone(),
        }
    }

    pub fn get_devices(&self) {
        let media = self.media.clone();
        let on_change = self.on_change.clone();
        let devices = self.devices.clone();
        spawn_local(async move {
            let new_devices = media.list_video().await;
            {
                *devices.borrow_mut() = new_devices;
            }
            on_change.emit(());
        });
    }
}

pub enum MinionAction {
    LeftCamChange(String),
    RightCamChange(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MinionState {
    pub cam_id: (String, String),
    pub devices: Vec<MediaDeviceInfo>,
}

use std::{cell::RefCell, rc::Rc};

use futures::join;
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use web_sys::{MediaDeviceInfo, MediaStream};
use yew::Callback;

use crate::services::{Media, Server, WebRtc};

pub struct MinionModel {
    on_change: Callback<()>,
    cam_id: (String, String),
    media: Media,
    webrtc: WebRtc,
    devices: Rc<RefCell<Vec<MediaDeviceInfo>>>,
    started: bool,
    streams: Rc<RefCell<(Option<MediaStream>, Option<MediaStream>)>>,
}

impl MinionModel {
    pub fn new(on_change: Callback<()>, server: Server) -> MinionModel {
        let cam_id = LocalStorage::get("minion_cam_id").unwrap_or_default();
        let media = Media::new();

        let model = MinionModel {
            on_change,
            cam_id,
            media,
            webrtc: WebRtc::new(server),
            devices: Rc::default(),
            started: false,
            streams: Rc::default(),
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
            MinionAction::StartSending => self.start_source(),
            MinionAction::StartReceiving => self.start_receiver(),
        }
    }

    pub fn state(&self) -> MinionState {
        MinionState {
            cam_id: self.cam_id.clone(),
            devices: self.devices.borrow().clone(),
            streams: self.streams.borrow().clone(),
            started: self.started,
        }
    }

    fn get_devices(&self) {
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

    fn start_source(&mut self) {
        self.started = true;
        self.on_change.emit(());

        let media = self.media.clone();
        let cam_id = self.cam_id.clone();
        let streams = self.streams.clone();
        let on_change = self.on_change.clone();
        let webrtc = self.webrtc.clone();
        spawn_local(async move {
            let left = media.get_user_video(&cam_id.0);
            let right = media.get_user_video(&cam_id.1);
            let new_streams = join!(left, right);
            {
                let mut streams_ref = streams.borrow_mut();
                streams_ref.0 = Some(new_streams.0.clone());
                streams_ref.1 = Some(new_streams.1.clone());
            }
            on_change.emit(());
            webrtc.send_video(new_streams).await;
        });
    }

    fn start_receiver(&mut self) {
        self.started = true;
        self.on_change.emit(());

        let streams = self.streams.clone();
        let on_change = self.on_change.clone();
        let webrtc = self.webrtc.clone();
        spawn_local(async move {
            let new_streams = webrtc.receive().await;
            {
                let mut streams_ref = streams.borrow_mut();
                streams_ref.0 = Some(new_streams.0.clone());
                streams_ref.1 = Some(new_streams.1.clone());
            }
            on_change.emit(());
        });
    }
}

pub enum MinionAction {
    LeftCamChange(String),
    RightCamChange(String),
    StartSending,
    StartReceiving,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MinionState {
    pub cam_id: (String, String),
    pub devices: Vec<MediaDeviceInfo>,
    pub streams: (Option<MediaStream>, Option<MediaStream>),
    pub started: bool,
}

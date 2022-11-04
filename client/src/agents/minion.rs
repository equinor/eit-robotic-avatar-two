use std::collections::HashSet;

use common::Tracking;
use futures::join;
use gloo_storage::{LocalStorage, Storage};
use web_sys::{MediaDeviceInfo, MediaStream};
use yew_agent::{Agent, AgentLink, Context, HandlerId};

use crate::services::{media, server, webrtc};

pub struct MinionAgent {
    cam_id: (String, String),
    devices: Vec<MediaDeviceInfo>,
    started: bool,
    streams: (Option<MediaStream>, Option<MediaStream>),
    sending: bool,
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl MinionAgent {
    fn send_state(&self) {
        for sub in self.subscribers.iter() {
            self.link.respond(
                *sub,
                MinionState {
                    cam_id: self.cam_id.clone(),
                    devices: self.devices.clone(),
                    streams: self.streams.clone(),
                    started: self.started,
                },
            );
        }
    }
}

impl Agent for MinionAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = MinionAction;
    type Output = MinionState;

    fn create(link: AgentLink<Self>) -> Self {
        let cam_id = LocalStorage::get("minion_cam_id").unwrap_or_default();
        link.send_future(async move { Msg::NewDevices(media::list_video().await) });

        MinionAgent {
            cam_id,
            devices: Vec::new(),
            started: false,
            streams: (None, None),
            sending: false,
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::NewDevices(devices) => {
                self.devices = devices;
                self.send_state();
            }
            Msg::SendVideo(left, right) => {
                self.streams = (Some(left.clone()), Some(right.clone()));
                self.send_state();
                self.link.send_future(async move {
                    webrtc::send_video((left, right)).await;
                    Msg::SendDone
                });
            }
            Msg::SendDone => {}
            Msg::ReceiverDone((left, right)) => {
                self.streams = (Some(left), Some(right));
                self.send_state();
            }
            Msg::ReadyToSend => {
                self.sending = false;
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            MinionAction::LeftCamChange(id) => {
                self.cam_id.0 = id;
                LocalStorage::set("minion_cam_id", self.cam_id.clone()).unwrap();
                self.send_state();
            }
            MinionAction::RightCamChange(id) => {
                self.cam_id.1 = id;
                LocalStorage::set("minion_cam_id", self.cam_id.clone()).unwrap();
                self.send_state();
            }
            MinionAction::StartSending => {
                self.started = true;

                let cam_id = self.cam_id.clone();
                self.link.send_future(async move {
                    let left = media::get_user_video(&cam_id.0);
                    let right = media::get_user_video(&cam_id.1);
                    let (left, right) = join!(left, right);
                    Msg::SendVideo(left, right)
                });
            }
            MinionAction::StartReceiving => {
                self.started = true;
                self.link
                    .send_future(async move { Msg::ReceiverDone(webrtc::receive().await) });
            }
            MinionAction::Tracking(value) => {
                if !self.sending {
                    self.sending = true;

                    self.link.send_future(async move {
                        server::post_minion_tracking(&value).await;
                        Msg::ReadyToSend
                    });
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

pub enum Msg {
    NewDevices(Vec<MediaDeviceInfo>),
    SendVideo(MediaStream, MediaStream),
    SendDone,
    ReceiverDone((MediaStream, MediaStream)),
    ReadyToSend,
}

pub enum MinionAction {
    LeftCamChange(String),
    RightCamChange(String),
    StartSending,
    StartReceiving,
    Tracking(Tracking),
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MinionState {
    pub cam_id: (String, String),
    pub devices: Vec<MediaDeviceInfo>,
    pub streams: (Option<MediaStream>, Option<MediaStream>),
    pub started: bool,
}

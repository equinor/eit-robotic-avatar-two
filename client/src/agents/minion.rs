use std::collections::HashSet;

use common::Tracking;
use web_sys::MediaDeviceInfo;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

use crate::services::{media, server};

pub struct MinionAgent {
    devices: Vec<MediaDeviceInfo>,
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
                    devices: self.devices.clone(),
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
        link.send_future(async move { Msg::NewDevices(media::list_video().await) });

        MinionAgent {
            devices: Vec::new(),
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
            Msg::ReadyToSend => {
                self.sending = false;
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
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
    ReadyToSend,
}

pub enum MinionAction {
    Tracking(Tracking),
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MinionState {
    pub devices: Vec<MediaDeviceInfo>,
}

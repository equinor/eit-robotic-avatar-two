use std::collections::HashSet;

use web_sys::{MediaDeviceInfo, MediaStream};
use yew_agent::{Agent, AgentLink, Context, HandlerId};

use crate::services::Media;

pub struct MediaAgent {
    media: Media,
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl MediaAgent {
    fn send_all(&self, state: &MediaState) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, state.clone());
        }
    }
}

impl Agent for MediaAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = MediaActions;
    type Output = MediaState;

    fn create(link: AgentLink<Self>) -> Self {
        MediaAgent {
            media: Media::new(),
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::SetState(state) => self.send_all(&state),
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            MediaActions::GetMedia => {
                let media = self.media.clone();
                self.link.send_future(async move {
                    Msg::SetState(MediaState {
                        left: media.get_user_video("").await,
                        devices: media.list_devices().await,
                    })
                })
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
    SetState(MediaState),
}

pub enum MediaActions {
    GetMedia,
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

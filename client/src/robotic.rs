mod media;

use crate::server::Server;

use self::media::MediaService;
pub use self::media::MediaState;
use common::SendMessage;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

pub enum RoboticMsg {
    Media,
    SendMessage(SendMessage),
}

pub struct Robotic {
    media: MediaService,
    server: Server,
}

impl Robotic {
    pub fn new(on_change: Callback<()>, server: Server) -> Robotic {
        Robotic {
            media: MediaService::new(on_change),
            server,
        }
    }

    pub fn state(&self) -> RoboticState {
        RoboticState {
            media: self.media.state(),
        }
    }

    pub fn action(&mut self, action: RoboticMsg) {
        match action {
            RoboticMsg::Media => self.media.get_media(),
            RoboticMsg::SendMessage(msg) => self.send_message(msg),
        }
    }

    fn send_message(&mut self, msg: SendMessage) {
        let server = self.server.clone();
        spawn_local(async move { server.post_message(&msg).await })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoboticState {
    pub media: MediaState,
}

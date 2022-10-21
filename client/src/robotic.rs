mod media;
pub mod server;

use self::media::MediaService;
pub use self::media::MediaState;
pub use server::send_message;
use yew::Callback;

pub enum RoboticMsg {
    Media,
}

pub struct Robotic {
    media: MediaService,
}

impl Robotic {
    pub fn new(on_change: Callback<()>) -> Robotic {
        Robotic {
            media: MediaService::new(on_change),
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
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoboticState {
    pub media: MediaState,
}

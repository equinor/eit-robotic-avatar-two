mod media;
mod robot;

use crate::server::Server;

pub use self::media::MediaState;
pub use self::robot::RobotState;

use self::{media::MediaService, robot::Robot};
use common::SendMessage;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

pub enum RoboticMsg {
    Media,
    SendMessage(SendMessage),
}

pub struct Robotic {
    media: MediaService,
    robot: Robot,
    server: Server,
}

impl Robotic {
    pub fn new(on_change: Callback<()>, server: Server) -> Robotic {
        Robotic {
            media: MediaService::new(on_change.clone()),
            robot: Robot::new(server.clone(), on_change),
            server,
        }
    }

    pub fn state(&self) -> RoboticState {
        RoboticState {
            media: self.media.state(),
            robot: self.robot.state(),
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
    pub robot: RobotState,
}

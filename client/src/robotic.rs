mod media;
mod minion;
mod robot;

use crate::services::Server;

pub use self::media::MediaState;
pub use self::minion::MinionState;
pub use self::robot::RobotState;

use self::{media::MediaModel, minion::Minion, robot::Robot};
use common::SendMessage;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

pub enum RoboticMsg {
    Media,
    SendMessage(SendMessage),
    GenRobotToken,
    GenPin,
}

pub struct Robotic {
    media: MediaModel,
    minion: Minion,
    robot: Robot,
    server: Server,
}

impl Robotic {
    pub fn new(on_change: Callback<()>, server: Server) -> Robotic {
        Robotic {
            media: MediaModel::new(on_change.clone()),
            minion: Minion::new(),
            robot: Robot::new(server.clone(), on_change),
            server,
        }
    }

    pub fn state(&self) -> RoboticState {
        RoboticState {
            media: self.media.state(),
            minion: self.minion.state(),
            robot: self.robot.state(),
        }
    }

    pub fn action(&mut self, action: RoboticMsg) {
        match action {
            RoboticMsg::Media => self.media.get_media(),
            RoboticMsg::SendMessage(msg) => self.send_message(msg),
            RoboticMsg::GenRobotToken => self.robot.gen_token(),
            RoboticMsg::GenPin => self.robot.gen_pin(),
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
    pub minion: MinionState,
    pub robot: RobotState,
}

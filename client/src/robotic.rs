pub mod media;
mod minion;
mod robot;

use crate::services::Server;

pub use self::media::MediaAgent;
pub use self::minion::MinionAction;
pub use self::minion::MinionState;
pub use self::robot::RobotState;

use self::{minion::MinionModel, robot::Robot};
use common::SendMessage;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

pub enum RoboticMsg {
    SendMessage(SendMessage),
    GenRobotToken,
    GenPin,
    Minion(MinionAction),
}

pub struct Robotic {
    minion: MinionModel,
    robot: Robot,
    server: Server,
}

impl Robotic {
    pub fn new(on_change: Callback<()>, server: Server) -> Robotic {
        Robotic {
            minion: MinionModel::new(on_change.clone(), server.clone()),
            robot: Robot::new(server.clone(), on_change),
            server,
        }
    }

    pub fn state(&self) -> RoboticState {
        RoboticState {
            minion: self.minion.state(),
            robot: self.robot.state(),
        }
    }

    pub fn action(&mut self, action: RoboticMsg) {
        match action {
            RoboticMsg::SendMessage(msg) => self.send_message(msg),
            RoboticMsg::GenRobotToken => self.robot.gen_token(),
            RoboticMsg::GenPin => self.robot.gen_pin(),
            RoboticMsg::Minion(action) => self.minion.action(action),
        }
    }

    fn send_message(&mut self, msg: SendMessage) {
        let server = self.server.clone();
        spawn_local(async move { server.post_message(&msg).await })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoboticState {
    pub minion: MinionState,
    pub robot: RobotState,
}

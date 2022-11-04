pub mod media;
pub mod minion;
mod robot;

use crate::services::Server;

pub use self::media::MediaAgent;
pub use self::minion::MinionAgent;
pub use self::robot::RobotState;

use self::robot::Robot;
use common::SendMessage;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

pub enum RoboticMsg {
    SendMessage(SendMessage),
    GenRobotToken,
    GenPin,
}

pub struct Robotic {
    robot: Robot,
    server: Server,
}

impl Robotic {
    pub fn new(on_change: Callback<()>, server: Server) -> Robotic {
        Robotic {
            robot: Robot::new(server.clone(), on_change),
            server,
        }
    }

    pub fn state(&self) -> RoboticState {
        RoboticState {
            robot: self.robot.state(),
        }
    }

    pub fn action(&mut self, action: RoboticMsg) {
        match action {
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
    pub robot: RobotState,
}

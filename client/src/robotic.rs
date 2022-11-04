pub mod media;
pub mod minion;
mod robot;

use crate::services::server;

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
}

impl Robotic {
    pub fn new(on_change: Callback<()>) -> Robotic {
        Robotic {
            robot: Robot::new(on_change),
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
        spawn_local(async move { server::post_message(&msg).await })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoboticState {
    pub robot: RobotState,
}

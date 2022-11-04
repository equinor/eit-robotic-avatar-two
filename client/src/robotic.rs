pub mod media;
pub mod minion;
pub mod robot;

use crate::services::server;

pub use self::media::MediaAgent;
pub use self::minion::MinionAgent;
pub use self::robot::RobotAgent;

use common::SendMessage;
use wasm_bindgen_futures::spawn_local;

pub enum RoboticMsg {
    SendMessage(SendMessage),
}

pub struct Robotic {

}

impl Robotic {
    pub fn new() -> Robotic {
        Robotic {
        }
    }

    pub fn action(&mut self, action: RoboticMsg) {
        match action {
            RoboticMsg::SendMessage(msg) => self.send_message(msg),
        }
    }

    fn send_message(&mut self, msg: SendMessage) {
        spawn_local(async move { server::post_message(&msg).await })
    }
}

impl Default for Robotic {
    fn default() -> Self {
        Self::new()
    }
}
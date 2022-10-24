use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotRegister {
    pub name: String,
    pub network_interfaces: Vec<Interface>,
}

/// Information for a networking interface
///
/// This interface information is only meant to be shown to users.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Interface {
    pub name: String,
    pub ip: String,
    pub broadcast: String,
    pub netmask: String,
    pub mac: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RobotConfig {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessage {
    pub topic: String,
    pub msg_type: String,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub topic: String,
    pub msg_type: String,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotStatus {
    pub last_seen: Option<OffsetDateTime>,
    pub interfaces: Vec<Interface>,
}

// Legacy types from the old robotic avatar

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Tracking {
    pub head: Head,
    pub drive: Drive,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Head {
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Drive {
    pub speed: f64,
    pub turn: f64,
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RobotRegister {
    pub name: String,
    pub network_interfaces: Vec<Interface>,
}

/// Information for a networking interface
///
/// This interface information is only meant to be shown to users.
#[derive(Debug, Serialize, Deserialize)]
pub struct Interface {
    pub name: String,
    pub ip: String,
    pub broadcast: String,
    pub netmask: String,
    pub mac: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RobotConfig {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessage {
    pub topic: String,
    pub msg_type: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub topic: String,
    pub msg_type: String,
    pub payload: Vec<u8>,
}

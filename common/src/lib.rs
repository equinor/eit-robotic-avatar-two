use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use url::Url;

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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RobotStatus {
    pub last_seen: Option<OffsetDateTime>,
    pub interfaces: Vec<Interface>,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Tracking {
    pub head: Head,
    pub drive: Drive,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Head {
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Drive {
    pub speed: f64,
    pub turn: f64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RtcMessage {
    pub left: RtcSession,
    pub right: RtcSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RtcSession {
    pub sdp_type: String,
    pub sdp: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RtcIce(pub Vec<Url>);

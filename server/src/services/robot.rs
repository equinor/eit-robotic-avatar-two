use brain::{RobotRegister, RobotStatus};
use parking_lot::Mutex;
use time::OffsetDateTime;

pub struct Robot {
    status: Mutex<RobotStatus>,
}

impl Robot {
    pub fn new() -> Robot {
        let status = RobotStatus {
            last_seen: None,
            interfaces: Vec::new(),
        };

        Robot {
            status: Mutex::new(status),
        }
    }

    pub fn status(&self) -> RobotStatus {
        self.status.lock().clone()
    }

    pub fn register(&self, register: RobotRegister) {
        let mut status = self.status.lock();
        status.last_seen = Some(OffsetDateTime::now_utc());
        status.interfaces = register.network_interfaces;
    }
}

impl Default for Robot {
    fn default() -> Self {
        Self::new()
    }
}

use common::RobotStatus;
use parking_lot::Mutex;
use time::OffsetDateTime;

pub struct Robot {
    last_seen: Mutex<Option<OffsetDateTime>>,
}

impl Robot {
    pub fn new() -> Robot {
        Robot {
            last_seen: Mutex::new(None),
        }
    }

    pub fn ping(&self) {
        *self.last_seen.lock() = Some(OffsetDateTime::now_utc())
    }

    pub fn status(&self) -> RobotStatus {
        RobotStatus {
            last_seen: *self.last_seen.lock(),
        }
    }
}

impl Default for Robot {
    fn default() -> Self {
        Self::new()
    }
}

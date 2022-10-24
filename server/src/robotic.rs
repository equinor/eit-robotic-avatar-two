mod messaging;
mod robot;

pub use messaging::Messaging;
pub use robot::Robot;

use std::sync::Arc;

#[derive(Clone)]
pub struct Robotic(Arc<Inner>);

struct Inner {
    messaging: Messaging,
    robot: Robot,
}

impl Robotic {
    pub fn new() -> Robotic {
        let inner = Inner {
            messaging: Messaging::new(),
            robot: Robot::new(),
        };
        Robotic(Arc::new(inner))
    }

    pub fn messaging(&self) -> &Messaging {
        &self.0.messaging
    }

    pub fn robot(&self) -> &Robot {
        &self.0.robot
    }
}

impl Default for Robotic {
    fn default() -> Self {
        Self::new()
    }
}

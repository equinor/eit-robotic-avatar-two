mod auth;
mod messaging;
mod minion;
mod robot;

pub use messaging::Messaging;
pub use minion::Minion;
pub use robot::Robot;

use anyhow::Result;
use std::sync::Arc;

use self::auth::Auth;
use crate::Config;

#[derive(Clone)]
pub struct Service(Arc<Inner>);

struct Inner {
    auth: Auth,
    messaging: Messaging,
    minion: Minion,
    robot: Robot,
}

impl Service {
    pub async fn new(config: &Config) -> Result<Service> {
        let inner = Inner {
            auth: Auth::new(config).await?,
            messaging: Messaging::new(),
            minion: Minion::new(),
            robot: Robot::new(),
        };
        Ok(Service(Arc::new(inner)))
    }

    pub fn auth(&self) -> &Auth {
        &self.0.auth
    }

    pub fn messaging(&self) -> &Messaging {
        &self.0.messaging
    }

    pub fn robot(&self) -> &Robot {
        &self.0.robot
    }

    pub fn minion(&self) -> &Minion {
        &self.0.minion
    }
}

mod auth;
mod messaging;
mod robot;

pub use messaging::Messaging;
pub use robot::Robot;

use anyhow::Result;
use std::sync::Arc;

use self::auth::Auth;
use crate::Config;

#[derive(Clone)]
pub struct Robotic(Arc<Inner>);

struct Inner {
    auth: Auth,
    messaging: Messaging,
    robot: Robot,
}

impl Robotic {
    pub async fn new(config: &Config) -> Result<Robotic> {
        let inner = Inner {
            auth: Auth::new(config).await?,
            messaging: Messaging::new(),
            robot: Robot::new(),
        };
        Ok(Robotic(Arc::new(inner)))
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
}

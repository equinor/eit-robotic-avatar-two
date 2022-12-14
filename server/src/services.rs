mod auth;
mod minion;
mod robot;

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
    minion: Minion,
    robot: Robot,
}

impl Service {
    pub async fn new(config: &Config) -> Result<Service> {
        let inner = Inner {
            auth: Auth::new(config).await?,
            minion: Minion::new(config.ice.to_owned()),
            robot: Robot::new(),
        };
        Ok(Service(Arc::new(inner)))
    }

    pub fn auth(&self) -> &Auth {
        &self.0.auth
    }

    pub fn robot(&self) -> &Robot {
        &self.0.robot
    }

    pub fn minion(&self) -> &Minion {
        &self.0.minion
    }
}

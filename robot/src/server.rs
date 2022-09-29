use anyhow::Result;
use common::{RobotConfig, RobotRegister};
use reqwest::Client;

use crate::config::LocalConfig;

pub struct Server {
    config: RobotConfig,
}

impl Server {
    pub async fn connect(config: LocalConfig, register: RobotRegister) -> Result<Server> {
        let client = Client::new();

        let config = client
            .post(config.base_url.join("api/robot/register")?)
            .json(&register)
            .send()
            .await?
            .json()
            .await?;

        Ok(Server { config })
    }

    pub fn config(&self) -> &RobotConfig {
        &self.config
    }
}

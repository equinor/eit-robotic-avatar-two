use anyhow::Result;
use common::{RobotConfig, RobotRegister, SendMessage};
use reqwest::{Client, Url};

use crate::config::LocalConfig;

pub struct Server {
    base_url: Url,
    client: Client,
    config: RobotConfig,
}

impl Server {
    pub async fn connect(config: LocalConfig, register: RobotRegister) -> Result<Server> {
        let client = Client::new();
        let base_url = config.base_url;

        let config = client
            .post(base_url.join("api/robot/register")?)
            .json(&register)
            .send()
            .await?
            .json()
            .await?;

        Ok(Server {
            base_url,
            client,
            config,
        })
    }

    pub fn config(&self) -> &RobotConfig {
        &self.config
    }

    pub async fn send_message(&self, msg: SendMessage) -> Result<()> {
        self.client
            .post(self.base_url.join("api/messaging")?)
            .json(&msg)
            .send()
            .await?;

        Ok(())
    }
}

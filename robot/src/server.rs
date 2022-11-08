use anyhow::Result;
use common::{RobotConfig, RobotRegister, SendMessage};
use reqwest::{header, Client, RequestBuilder, Url};

use crate::config::LocalConfig;

pub struct Server {
    base_url: Url,
    client: Client,
    config: RobotConfig,
}

impl Server {
    pub async fn connect(config: LocalConfig, register: RobotRegister) -> Result<Server> {
        let mut headers = header::HeaderMap::new();
        let mut auth_value =
            header::HeaderValue::from_str(&("Bearer ".to_owned() + &config.token)).unwrap();
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        let client = Client::builder().default_headers(headers).build().unwrap();
        let base_url = config.server_url;

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

    pub fn get(&self, path: &str) -> RequestBuilder {
        self.client.get(self.base_url.join(path).unwrap())
    }
}

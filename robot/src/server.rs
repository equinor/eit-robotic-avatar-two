use anyhow::Result;
use common::{RobotConfig, RobotRegister};
use reqwest::{header, Client};

use crate::config::LocalConfig;

pub struct Server {
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

        let config = client
            .post(config.server_url.join("api/robot/register")?)
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

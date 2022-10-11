use anyhow::{Context, Result};
use dotenvy::dotenv;
use reqwest::Url;
use std::env;

/// Local robot config from environment.
///
/// Minium needed to contact central control server.
/// All the other settings should be gotten from server.
pub struct LocalConfig {
    pub server: Url, // Address to be used by UDP server.
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            server: Url::parse("http://localhost/").unwrap(),
        }
    }
}

impl LocalConfig {
    pub fn from_env() -> Result<LocalConfig> {
        dotenv().ok();

        const MINION_SERVER: &str = "MINION_SERVER";
        let server = env::var(MINION_SERVER).context(format!(
            "{} env variable must be set to the server url",
            MINION_SERVER
        ))?;
        let server = Url::parse(&server).context(format!(
            "{} needs to be a valid URL got {} ",
            MINION_SERVER, server
        ))?;

        Ok(LocalConfig { server })
    }
}

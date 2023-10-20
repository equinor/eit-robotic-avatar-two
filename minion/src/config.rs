use std::env;

use anyhow::{Context, Result};
use dotenvy::dotenv;
use reqwest::Url;

pub struct Config {
    pub server_url: Url,
    pub token: String,
}

impl Config {
    fn from_env() -> Result<Config> {
        dotenv().ok();

        let server_url = env::var("ROBOT_SERVER_URL").context("Environment variable ROBOT_SERVER_URL was not found. Please set with the url to the server.")?;

        let token = env::var("ROBOT_TOKEN").context("Environment variable ROBOT_TOKEN was not found. Please set with a valid authentication token.")?;

        Ok(Config {
            server_url: Url::parse(&server_url).context("Server url mut be a valid url")?,
            token,
        })
    }

    pub fn get_config() -> Config {
        let config = Config::from_env();
        #[cfg(debug_assertions)]
        let config = config.unwrap_or_default();
        #[cfg(not(debug_assertions))]
        let config = config.unwrap();
        config
    }
}

#[cfg(debug_assertions)]
impl Default for Config {
    fn default() -> Self {
        Self {
            server_url: Url::parse("http://127.0.0.1:3000/").expect("Default base_url is not valid"),
            // This token is signed by the dev-key that is an empty string.
            // No release build of the server will accept this key.
            token: "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJyb2JvdCIsImlhdCI6MTY2NjY4OTQ5Nn0.do5Rlx3DaAaYY1ufMpzuz3fjyw9_NR7p1YD3BKM-2Zs".to_owned()
        }
    }
}

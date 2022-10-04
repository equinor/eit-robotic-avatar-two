use std::env::args;

use anyhow::{anyhow, Context, Result};
use reqwest::Url;

pub struct LocalConfig {
    pub server_url: Url,
    pub token: String,
}

impl LocalConfig {
    pub fn from_args() -> Result<LocalConfig> {
        let args: Vec<_> = args().collect();
        let server_url = args.get(1).ok_or_else(|| {
            anyhow!("No base_url was passed in as argument 'robot <base_url> <token>' ")
        })?;

        let token = args.get(2).ok_or_else(|| {
            anyhow!("No token was passed in as argument 'robot <base_url> <token>' ")
        })?;

        println!("{:?}", args);
        Ok(LocalConfig {
            server_url: Url::parse(server_url).context("Base_url argument mut be a valid url")?,
            token: token.to_owned(),
        })
    }
}

#[cfg(debug_assertions)]
impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            server_url: Url::parse("http://127.0.0.1:3000/").expect("Default base_url is not valid"),
            // This token is singed by the dev-key that is an empty string.
            // No release build of the server will accept this key.
            token: "eyJzdWIiOiJyb2JvdCIsImlhdCI6MTUxNjIzOTAyMn0.eUcwVDVwKmEnxo8uGej7C7Sc93IBcDdkfuQ0qOTOs5M".to_owned()
        }
    }
}

use std::env::args;

use anyhow::{anyhow, Context, Result};
use reqwest::Url;

pub struct LocalConfig {
    pub base_url: Url,
}

impl LocalConfig {
    pub fn from_args() -> Result<LocalConfig> {
        let args: Vec<_> = args().collect();
        let base_url = args
            .get(1)
            .ok_or_else(|| anyhow!("No base_url was pased in as argument 'robot <base_url>' "))?;

        println!("{:?}", args);
        Ok(LocalConfig {
            base_url: Url::parse(base_url).context("Base_url agrument mut be a valid url")?,
        })
    }
}

#[cfg(debug_assertions)]
impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            base_url: Url::parse("http://127.0.0.1:3000/").expect("Default base_url is not valid"),
        }
    }
}

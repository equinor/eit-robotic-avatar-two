use std::net::SocketAddr;

use anyhow::{Ok, Result};

pub struct Config {
    pub bind_address: SocketAddr,
}

impl Config {
    pub(crate) fn load() -> Result<Config> {
        let config = Config {
            bind_address: "127.0.0.1:3000"
                .parse()
                .expect("Invalid hardcoded bind_address."),
        };

        Ok(config)
    }
}

use std::{env, net::SocketAddr};

use anyhow::{Ok, Result};
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub struct Config {
    pub bind_address: SocketAddr,
    pub token_key: Hmac<Sha256>,
}

impl Config {
    pub(crate) fn load() -> Result<Config> {
        let key_string = env::var("AVATAR_TOKEN_KEY").unwrap_or_default();
        let key = validate_key(&key_string)?;

        let config = Config {
            bind_address: "127.0.0.1:3000"
                .parse()
                .expect("Invalid hardcoded bind_address."),
            token_key: key,
        };

        Ok(config)
    }
}

#[cfg(not(debug_assertions))]
fn validate_key(key: &str) -> Result<Hmac<Sha256>> {
    use anyhow::bail;

    if key.len() < 32 {
        bail!("AVATAR_TOKEN_KEY must be at least 32 bytes long!")
    }

    Ok(Hmac::new_from_slice(key.as_bytes()).unwrap())
}

#[cfg(debug_assertions)]
fn validate_key(key: &str) -> Result<Hmac<Sha256>> {
    Ok(Hmac::new_from_slice(key.as_bytes()).unwrap())
}

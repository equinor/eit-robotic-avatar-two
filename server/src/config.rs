use std::{env, net::SocketAddr};

use anyhow::{Context, Ok, Result};
use hmac::{Hmac, Mac};
use log::warn;
use reqwest::Url;
use sha2::Sha256;

pub struct Config {
    pub bind_address: SocketAddr,
    pub token_key: Hmac<Sha256>,
    pub azure_ad: Option<AzureAdConfig>,
}

impl Config {
    pub fn load() -> Result<Config> {
        let key_string = env::var("AVATAR_TOKEN_KEY").unwrap_or_default();
        let key = validate_key(&key_string)?;

        let bind_address =
            env::var("AVATAR_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
        let bind_address = bind_address
            .parse()
            .context("Failed to parse AVATAR_BIND_ADDRESS")?;

        let config = Config {
            bind_address,
            token_key: key,
            azure_ad: AzureAdConfig::load()?,
        };

        Ok(config)
    }
}

pub struct AzureAdConfig {
    pub id: String,
    pub secret: String,
    pub url: Url,
    pub base: Url,
}

impl AzureAdConfig {
    pub fn load() -> Result<Option<AzureAdConfig>> {
        let id = env::var("AVATAR_AZURE_AD_ID").ok();
        Ok(if let Some(id) = id {
            let secret = env::var("AVATAR_AZURE_AD_SECRET")
                .context("AVATAR_AZURE_AD_SECRET needs to be set to azure ad client secret")?;
            let url = env::var("AVATAR_AZURE_AD_URL")
                .context("AVATAR_AZURE_AD_URL needs to be set to a valid url")?;
            let url = Url::parse(&url).context("AVATAR_AZURE_AD_URL was not a valid url")?;

            let base = env::var("AVATAR_AZURE_AD_BASE_URL").ok();
            // Try Radix config
            let base = base.or_else(AzureAdConfig::base_from_radix);
            // If all fail debug default
            let base = base.unwrap_or_else(|| "http://localhost:8080/".to_string());
            let base = Url::parse(&base).context("AVATAR_AZURE_AD_BASE_URL was not a valid url")?;

            Some(AzureAdConfig {
                id,
                secret,
                url,
                base,
            })
        } else {
            warn!("Azure AD is not configured.");
            None
        })
    }

    fn base_from_radix() -> Option<String> {
        let radix = env::var("RADIX_PUBLIC_DOMAIN_NAME").ok()?;
        Some(format!("https://{}/", radix))
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

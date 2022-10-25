mod azure_ad;

use anyhow::{anyhow, Result};
use hmac::Hmac;
use jwt::{RegisteredClaims, SignWithKey, VerifyWithKey};
use reqwest::Url;
use sha2::Sha256;
use time::OffsetDateTime;

use crate::Config;

use self::azure_ad::AzureAd;

pub struct Auth {
    key: Hmac<Sha256>,
    azure_ad: Option<AzureAd>,
}

impl Auth {
    pub async fn new(config: &Config) -> Result<Auth> {
        let azure_ad = if let Some(config) = &config.azure_ad {
            Some(AzureAd::new(config).await?)
        } else {
            None
        };

        Ok(Auth {
            key: config.token_key.clone(),
            azure_ad,
        })
    }

    pub fn verify_header(&self, header: Option<&str>) -> bool {
        let token_string = header.and_then(|s| s.strip_prefix("Bearer "));
        if let Some(token) = token_string {
            let claims: Result<serde_json::Value, _> = token.verify_with_key(&self.key);
            claims.is_ok()
        } else {
            false
        }
    }

    pub fn gen_login(&self) -> Option<Url> {
        self.azure_ad.as_ref().map(|azure_ad| azure_ad.login_url())
    }

    pub fn gen_token_for_robot(&self) -> Result<String> {
        self.token_from_name("robot".to_string())
    }

    pub async fn token_from_azure_ad(&self, code: String, state: String) -> Result<String> {
        let azure_ad = self
            .azure_ad
            .as_ref()
            .ok_or_else(|| anyhow!("Azure AD is not configured."))?;

        self.token_from_name(azure_ad.name_from_reply(code, state).await?)
    }

    fn token_from_name(&self, name: String) -> Result<String> {
        let claims = RegisteredClaims {
            subject: Some(name),
            issued_at: Some(OffsetDateTime::now_utc().unix_timestamp() as u64),
            ..Default::default()
        };
        Ok(claims.sign_with_key(&self.key)?)
    }
}

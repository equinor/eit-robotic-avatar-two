mod azure_ad;

use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use hmac::Hmac;
use jwt::{RegisteredClaims, SignWithKey, VerifyWithKey};
use parking_lot::Mutex;
use rand::{distributions::Slice, Rng};
use reqwest::Url;
use sha2::Sha256;
use time::OffsetDateTime;

use crate::Config;

use self::azure_ad::AzureAd;

pub struct Auth {
    key: Hmac<Sha256>,
    azure_ad: Option<AzureAd>,
    pins: Mutex<Vec<(String, Instant)>>,
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
            pins: Mutex::default(),
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

    pub fn issue_pin(&self) -> String {
        let pin = gen_pin();
        let mut pins = self.pins.lock();
        pins.push((pin.clone(), Instant::now()));
        pin
    }

    pub fn token_from_pin(&self, pin: String) -> Result<String> {
        if self.valid_pin(pin) {
            self.token_from_name("FromPin".to_string())
        } else {
            Err(anyhow!("Pin not found."))
        }
    }

    fn token_from_name(&self, name: String) -> Result<String> {
        let claims = RegisteredClaims {
            subject: Some(name),
            issued_at: Some(OffsetDateTime::now_utc().unix_timestamp() as u64),
            ..Default::default()
        };
        Ok(claims.sign_with_key(&self.key)?)
    }

    fn valid_pin(&self, pin: String) -> bool {
        if debug_pin(&pin) {
            return true;
        }

        let mut pins = self.pins.lock();

        // Remove old pins
        const TIMEOUT: Duration = Duration::from_secs(5 * 60);
        let now = Instant::now();
        pins.retain(|(_, timestamp)| {
            let age = now.duration_since(*timestamp);
            age < TIMEOUT
        });

        // Find the pin and remove it if its there.
        let pin = pins
            .iter()
            .position(|(p, _)| p == &pin)
            .map(|index| pins.swap_remove(index));

        pin.is_some()
    }
}

fn gen_pin() -> String {
    const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let numbers_dist = Slice::new(&NUMBERS).unwrap();
    let rng = rand::thread_rng();
    rng.sample_iter(&numbers_dist).take(5).collect()
}

#[cfg(not(debug_assertions))]
fn debug_pin(pin: &str) -> bool {
    // In prod there are no debug pins.
    false
}

#[cfg(debug_assertions)]
fn debug_pin(pin: &str) -> bool {
    // In debug mode the pin 00000 is always valid.
    pin == "00000"
}

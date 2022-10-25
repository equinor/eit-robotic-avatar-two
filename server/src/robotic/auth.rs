use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{anyhow, Result};
use hmac::Hmac;
use jwt::{RegisteredClaims, SignWithKey, VerifyWithKey};

use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    AccessTokenHash, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse,
};
use parking_lot::Mutex;
use reqwest::Url;
use sha2::Sha256;
use time::OffsetDateTime;

use crate::Config;

pub struct Auth {
    key: Hmac<Sha256>,
    client: Option<CoreClient>,
    state: Arc<Mutex<Vec<AuthState>>>,
}

impl Auth {
    pub async fn new(config: &Config) -> Result<Auth> {
        let client = if let Some(ad_config) = &config.azure_ad {
            let provider_metadata = CoreProviderMetadata::discover_async(
                IssuerUrl::from_url(ad_config.url.clone()),
                async_http_client,
            )
            .await?;

            let client_id = ClientId::new(ad_config.id.clone());
            let client_secret = Some(ClientSecret::new(ad_config.secret.clone()));
            let redirect_url = RedirectUrl::from_url(
                ad_config
                    .base
                    .join("api/auth/azure_ad")
                    .expect("Invalid redirect URL"),
            );

            Some(
                CoreClient::from_provider_metadata(provider_metadata, client_id, client_secret)
                    .set_redirect_uri(redirect_url),
            )
        } else {
            None
        };

        Ok(Auth {
            key: config.token_key.clone(),
            client,
            state: Arc::default(),
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
        if let Some(client) = &self.client {
            let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
            let (url, token, nonce) = client
                .authorize_url(
                    CoreAuthenticationFlow::AuthorizationCode,
                    CsrfToken::new_random,
                    Nonce::new_random,
                )
                .set_pkce_challenge(pkce_code_challenge)
                .add_scope(Scope::new("email".to_string()))
                .add_scope(Scope::new("profile".to_string()))
                .url();

            let mut state = self.state.lock();
            state.push(AuthState {
                pkce: pkce_code_verifier,
                nonce,
                token,
                timestamp: Instant::now(),
            });

            Some(url)
        } else {
            None
        }
    }

    pub async fn token_from_azure_ad(&self, code: String, state: String) -> Result<String> {
        let state = self.get_state(&state).ok_or_else(|| {
            anyhow!("Did not find the record from a previous url generation. Timed out maybe")
        })?;
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("Azure AD is not configured."))?;
        let code = AuthorizationCode::new(code);
        let token_response = client
            .exchange_code(code)
            .set_pkce_verifier(state.pkce)
            .request_async(async_http_client)
            .await?;

        let id_token = token_response
            .id_token()
            .ok_or_else(|| anyhow!("Server did not return an ID token"))?;
        let claims = id_token.claims(&client.id_token_verifier(), &state.nonce)?;

        if let Some(expected_access_token_hash) = claims.access_token_hash() {
            let actual_access_token_hash = AccessTokenHash::from_token(
                token_response.access_token(),
                &id_token.signing_alg()?,
            )?;
            if actual_access_token_hash != *expected_access_token_hash {
                return Err(anyhow!("Invalid access token"));
            }
        }

        let name = claims
            .name()
            .and_then(|l| l.get(None))
            .map(|n| n.to_string())
            .ok_or_else(|| anyhow!("No name in OpenID Token"))?;

        // Create and sign the token
        let claims = RegisteredClaims {
            subject: Some(name),
            issued_at: Some(OffsetDateTime::now_utc().unix_timestamp() as u64),
            ..Default::default()
        };
        Ok(claims.sign_with_key(&self.key)?)
    }

    fn get_state(&self, token: &str) -> Option<AuthState> {
        let mut state = self.state.lock();

        // Remove old login state
        const TIMEOUT: Duration = Duration::from_secs(5 * 60);
        let now = Instant::now();
        state.retain(|state| {
            let age = now.duration_since(state.timestamp);
            age < TIMEOUT
        });

        state
            .iter()
            .position(|s| s.token.secret() == token)
            .map(|index| state.swap_remove(index))
    }
}

struct AuthState {
    pub pkce: PkceCodeVerifier,
    pub nonce: Nonce,
    pub token: CsrfToken,
    pub timestamp: Instant,
}

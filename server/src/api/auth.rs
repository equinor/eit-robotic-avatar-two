use std::{sync::Arc, time::Instant};

use anyhow::Result;
use axum::{
    http::{self, Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Extension, Router,
};
use hmac::Hmac;
use jwt::VerifyWithKey;
use log::warn;
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, Scope,
};
use parking_lot::Mutex;
use reqwest::Url;
use sha2::Sha256;

use crate::Config;

pub async fn routes(router: Router, config: &Config) -> Result<Router> {
    let auth = Auth::new(config).await?;

    let key = auth.key.clone();
    let router = router
        .route_layer(middleware::from_fn(move |req, next| {
            middleware(req, next, key.clone())
        }))
        .route("/api/auth/login", get(login_handler))
        .layer(Extension(auth));

    Ok(router)
}

async fn login_handler(Extension(mut auth): Extension<Auth>) -> String {
    let url = auth.gen_login().map(|u| u.to_string());
    url.unwrap_or_default()
}

#[derive(Debug)]
struct AuthState {
    pub _pkce: PkceCodeVerifier,
    pub _nonce: Nonce,
    pub _token: CsrfToken,
    pub _timestamp: Instant,
}

#[derive(Debug, Clone)]
struct Auth {
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

    pub fn gen_login(&mut self) -> Option<Url> {
        if let Some(client) = &self.client {
            let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
            let (url, _token, _nonce) = client
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
                _pkce: pkce_code_verifier,
                _nonce,
                _token,
                _timestamp: Instant::now(),
            });

            Some(url)
        } else {
            None
        }
    }
}

pub async fn middleware<B>(
    req: Request<B>,
    next: Next<B>,
    key: Hmac<Sha256>,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token_string = header_to_token(auth_header);
    if !verify_token(token_string, &key) {
        warn!("Unauthenticated REQUEST!!!")
    }

    // For now everything is authenticated
    Ok(next.run(req).await)
}

/// Tries to extract
fn header_to_token(header: Option<&str>) -> Option<String> {
    // The header is always the same so no need to send it to the client.
    const JWT_HEADER: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.";

    let header = header.and_then(|s| s.strip_prefix("Bearer "));

    header.map(|token_body| format!("{}{}", JWT_HEADER, token_body))
}

fn verify_token(token_string: Option<String>, key: &Hmac<Sha256>) -> bool {
    if let Some(token) = token_string {
        let claims: Result<serde_json::Value, _> = token.verify_with_key(key);
        claims.is_ok()
    } else {
        false
    }
}

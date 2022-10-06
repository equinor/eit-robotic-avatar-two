use axum::{
    http::{self, Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    Router,
};
use hmac::Hmac;
use jwt::VerifyWithKey;
use log::warn;
use sha2::Sha256;

use crate::Config;

pub fn routes(router: Router, config: &Config) -> Router {
    let auth = Auth::new(config);
    router.route_layer(middleware::from_fn(move |req, next| {
        let auth = auth.clone();
        auth.middleware(req, next)
    }))
}

#[derive(Debug, Clone)]
struct Auth {
    key: Hmac<Sha256>,
}

impl Auth {
    pub fn new(config: &Config) -> Auth {
        Auth {
            key: config.token_key.clone(),
        }
    }

    pub async fn middleware<B>(
        self,
        req: Request<B>,
        next: Next<B>,
    ) -> Result<Response, StatusCode> {
        let auth_header = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        let token_string = header_to_token(auth_header);
        if !self.verify_token(token_string) {
            warn!("Unauthenticated REQUEST!!!")
        }

        // For now everything is authenticated
        Ok(next.run(req).await)
    }

    fn verify_token(&self, token_string: Option<String>) -> bool {
        if let Some(token) = token_string {
            let claims: Result<serde_json::Value, _> = token.verify_with_key(&self.key);
            claims.is_ok()
        } else {
            false
        }
    }
}

/// Tries to extract
fn header_to_token(header: Option<&str>) -> Option<String> {
    // The header is always the same now need to send that to the client.
    const JWT_HEADER: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.";

    let header = header.and_then(|s| s.strip_prefix("Bearer "));

    header.map(|token_body| format!("{}{}", JWT_HEADER, token_body))
}

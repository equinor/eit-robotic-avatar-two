use anyhow::Result;
use axum::{
    extract::Query,
    http::{self, Request, StatusCode},
    middleware::{self, Next},
    response::{Redirect, Response},
    routing::get,
    Extension, Router,
};
use log::warn;
use serde::Deserialize;

use crate::Robotic;

pub fn routes(router: Router) -> Router {
    router
        .route_layer(middleware::from_fn(middleware))
        .route("/api/auth/login", get(login_handler))
        .route("/api/auth/azure_ad", get(azure_ad_handler))
}

async fn login_handler(Extension(service): Extension<Robotic>) -> String {
    let url = service.auth().gen_login().map(|u| u.to_string());
    url.unwrap_or_default()
}

#[derive(Deserialize)]
pub struct AuthQuery {
    pub code: String,
    pub state: String,
}

async fn azure_ad_handler(
    Extension(service): Extension<Robotic>,
    Query(query): Query<AuthQuery>,
) -> Redirect {
    let token = match service
        .auth()
        .token_from_azure_ad(query.code, query.state)
        .await
    {
        Ok(token) => token,
        Err(err) => {
            warn!("/api/auth/azure_ad: {}", err);
            String::new()
        }
    };
    let url = format!("/?token={}", token);
    Redirect::temporary(&url)
}

pub async fn middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let service: &Robotic = req
        .extensions()
        .get()
        .expect("No Robotic Service in request.");

    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if !service.auth().verify_header(auth_header) {
        warn!("Unauthenticated REQUEST!!!")
    }

    // For now everything is authenticated
    Ok(next.run(req).await)
}

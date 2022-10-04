mod auth;
mod config;
mod messaging;
mod robot;
mod server;

use anyhow::Result;
use axum::middleware;
use axum::Router;
use dotenvy::dotenv;
use log::debug;

use crate::auth::Auth;
pub use crate::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env files
    dotenv().ok();

    // Setup log.
    env_logger::init();

    debug!("Loading config");
    let config = Config::load()?;

    debug!("Loading modules");
    let auth = Auth::new(&config);
    let robot = robot::setup().await?;
    let messaging = messaging::setup().await?;

    debug!("Setting up routes");
    let auth_sub = auth.clone();
    let app = Router::new()
        .nest("/api/robot", robot)
        .nest("/api/messaging", messaging)
        .route_layer(middleware::from_fn(move |req, next| {
            let auth = auth_sub.clone();
            auth.middleware(req, next)
        }));

    debug!("Starting the server");
    server::serve(app, &config).await
}

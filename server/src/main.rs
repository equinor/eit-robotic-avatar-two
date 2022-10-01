mod config;
mod messaging;
mod robot;
mod server;

use anyhow::Result;
use axum::Router;
use dotenvy::dotenv;
use log::debug;

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
    let robot = robot::setup().await?;
    let messaging = messaging::setup().await?;

    debug!("Setting up routes");
    let app = Router::new()
        .nest("/api/robot", robot)
        .nest("/api/messaging", messaging);

    debug!("Starting the server");
    server::serve(app, &config).await
}

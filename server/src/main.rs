mod config;
mod robot;
mod server;

use anyhow::Result;
use axum::Router;
use log::debug;

pub use crate::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup log. 
    env_logger::init();

    debug!("Loading config");
    let config = Config::load()?;

    debug!("Loading modules");
    let robot = robot::setup().await?;

    debug!("Setting up routes");
    let app = Router::new()
        .nest("/api/robot", robot); 

    debug!("Starting the server");
    server::serve(app, &config).await
}

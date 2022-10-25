mod api;
mod config;
mod robotic;

use anyhow::Result;
use dotenvy::dotenv;
use log::debug;

pub use crate::config::Config;
pub use crate::robotic::Robotic;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env files
    dotenv().ok();

    // Setup log.
    env_logger::init();

    debug!("Loading config");
    let config = Config::load()?;

    debug!("Loading App");
    let robotic = Robotic::new(&config).await?;

    debug!("Stating API Server");
    api::serve(config, robotic).await
}

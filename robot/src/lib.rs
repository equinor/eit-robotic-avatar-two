mod config;
mod network;
mod server;

use std::process::exit;

use anyhow::Result;
use common::RobotRegister;
use config::LocalConfig;
use log::error;
use network::get_networking_interfaces;
use server::Server;

pub async fn setup() -> Server {
    // Setup log.
    env_logger::init();

    setup_failable().await.unwrap_or_else(|e| {
        error!("Startup failed: {}", e);
        exit(1);
    })
}

async fn setup_failable() -> Result<Server> {
    let config = LocalConfig::from_args();

    #[cfg(debug_assertions)]
    let config = config.unwrap_or_default();
    #[cfg(not(debug_assertions))]
    let config = config?;

    let register = RobotRegister {
        name: "robot".to_string(),
        network_interfaces: get_networking_interfaces(),
    };

    Server::connect(config, register).await
}

mod config;
mod server;

use std::process::exit;

use common::{RobotRegister, Interface};
use config::LocalConfig;
use log::error;
use anyhow::Result;
use network_interface::{NetworkInterface, NetworkInterfaceConfig, Addr};
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

    let network_interfaces = NetworkInterface::show()?;
    let interfaces = network_interfaces.iter().map(|i| {
        let mut interface = Interface {
            name: i.name.clone(),
            ip: Default::default(),
            broadcast: Default::default(),
            netmask: Default::default(),
            mac: i.mac_addr.clone().unwrap_or_default()
        };

        if let Some(Addr::V4(addr)) = i.addr{
            interface.ip = addr.ip.to_string();
            if let Some(broadcast) = addr.broadcast {
                interface.broadcast = broadcast.to_string();
            }
            if let Some(netmask) = addr.netmask {
                interface.netmask = netmask.to_string();
            }
        }

        if let Some(Addr::V6(addr)) = i.addr{
            interface.ip = addr.ip.to_string();
            if let Some(broadcast) = addr.broadcast {
                interface.broadcast = broadcast.to_string();
            }
            if let Some(netmask) = addr.netmask {
                interface.netmask = netmask.to_string();
            }
        }
        interface
    }).collect();

    

    let register = RobotRegister{
        name: "robot".to_string(),
        network_interfaces: interfaces
    };

    Server::connect(config, register).await
}
mod arm;
mod config;
mod drive;
mod network;
mod server;
mod tracking;

use std::time::Duration;

use arm::{arm_run, arm_start};

use common::RobotRegister;
use config::LocalConfig;
use drive::{drive_run, drive_start};
use network::get_networking_interfaces;
use server::Server;
use tokio::{runtime::Builder, signal, task};

fn main() {
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    runtime.spawn(start());

    runtime.block_on(signal::ctrl_c()).unwrap();
    runtime.shutdown_timeout(Duration::from_millis(100));
}

async fn start() {
    let config = LocalConfig::from_env();
    #[cfg(debug_assertions)]
    let config = config.unwrap_or_default();
    #[cfg(not(debug_assertions))]
    let config = config.unwrap();

    let register = RobotRegister {
        name: "Minion".to_string(),
        network_interfaces: get_networking_interfaces(),
    };

    let server = Server::connect(config, register).await.unwrap();
    let tracking = tracking::tracking(server);

    let mut drive = drive_start();
    {
        let tracking = tracking.clone();
        task::spawn_blocking(move || loop {
            let drive_tracking = { tracking.borrow().drive };
            drive_run(&mut drive, drive_tracking);
        });
    }

    let arm = arm_start();
    task::spawn_blocking(move || loop {
        let head = { tracking.borrow().head };
        //println!("Head: {:?}", head);
        arm_run(&arm, head);
    });
}

mod arm;
mod config;
mod drive;
mod server;

use arm::{arm_run, arm_start};
use config::LocalConfig;

use drive::{drive_run, drive_start};
use server::Server;
use std::thread;

use anyhow::Result;

fn main() -> Result<()> {
    let config = LocalConfig::from_env()?;

    let server = Server::new(&config)?;
    server.start();

    let mut drive = drive_start();
    let arm = arm_start();

    // Disable drive for now

    let server_drive = server.clone();
    thread::spawn(move || loop {
        drive_run(&mut drive, server_drive.drive());
    });

    loop {
        arm_run(&arm, server.head());
    }
}
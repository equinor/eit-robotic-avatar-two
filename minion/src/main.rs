#[cfg(feature = "python")]
mod arm;
mod config;
mod drive;
mod server;
mod tracking;

use std::time::Duration;

use config::Config;
use drive::{drive_run, drive_start};
use server::Server;
use tokio::{runtime::Builder, signal, task};

fn main() {
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    runtime.spawn(start());

    runtime.block_on(signal::ctrl_c()).unwrap();
    runtime.shutdown_timeout(Duration::from_millis(100));
}

async fn start() {
    let config = Config::get_config();
    let server = Server::connect(config).await;

    let tracking = tracking::tracking(server);

    let mut drive = drive_start();
    {
        let tracking = tracking.clone();
        task::spawn_blocking(move || loop {
            let drive_tracking = { tracking.borrow().drive };
            drive_run(&mut drive, drive_tracking);
        });
    }

    #[cfg(feature = "python")]
    {
        let arm = arm::arm_start();
        task::spawn_blocking(move || loop {
            let head = { tracking.borrow().head };
            //println!("Head: {:?}", head);
            arm::arm_run(&arm, head);
        });
    }
}

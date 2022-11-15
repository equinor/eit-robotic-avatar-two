mod arm;
mod drive;
mod tracking;

use std::time::Duration;

use arm::{arm_run, arm_start};

use drive::{drive_run, drive_start};
use tokio::{runtime::Builder, signal, task};

fn main() {
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    runtime.spawn(async {
        let server = robot::setup().await;
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
            println!("Head: {:?}", head);
            arm_run(&arm, head);
        });
    });

    runtime.block_on(signal::ctrl_c()).unwrap();
    runtime.shutdown_timeout(Duration::from_millis(100));
}

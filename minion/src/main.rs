mod arm;
mod drive;
mod tracking;

use arm::{arm_run, arm_start};

use drive::{drive_run, drive_start};
use tokio::task;

use anyhow::{Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let server = robot::setup().await;
    let tracking = tracking::tracking(server);

    let mut drive = drive_start();
    let arm = arm_start();

    {
        let tracking = tracking.clone();
        task::spawn_blocking(move || loop {
            let drive_tracking = { tracking.borrow().drive };
            drive_run(&mut drive, drive_tracking);
        });
    }

    task::spawn_blocking(move || {
        let head = { tracking.borrow().head };
        loop {
            arm_run(&arm, head);
        }
    })
    .await?;

    Ok(())
}

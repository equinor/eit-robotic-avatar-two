#[cfg(feature = "python")]
mod arm;
mod config;
mod server;
mod tracking;

use avatar::{arduino, controller, ControllerInputs, ControllerOutputs};
use config::Config;
use server::Server;
use tokio::{signal, task};

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = start() => (),
        _ = signal::ctrl_c() => ()
    }
}

async fn start() {
    let config = Config::get_config();
    let server = Server::connect(config).await;

    let tracking = tracking::tracking(server);

    let ControllerOutputs {
        drive: drive_receive,
        arm: arm_receive,
    } = controller(ControllerInputs { pilot: tracking });

    arduino::register(drive_receive);

    #[cfg(feature = "python")]
    {
        let arm = arm::arm_start();
        task::spawn_blocking(move || loop {
            let head = { *arm_receive.borrow() };
            //println!("Head: {:?}", head);
            arm::arm_run(&arm, head);
        });
    }
}

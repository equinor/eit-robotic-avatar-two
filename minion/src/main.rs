mod config;
mod server;
mod tracking;

use minion::{arduino, controller, ros, ControllerInputs, ControllerOutputs};
use config::Config;
use server::Server;
use tokio::signal;

#[tokio::main]
async fn main() {
    start().await;
    signal::ctrl_c().await.unwrap();
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
    ros::register(arm_receive);
}

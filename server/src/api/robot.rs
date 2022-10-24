use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use common::{RobotConfig, RobotRegister, RobotStatus};
use log::info;

use crate::Robotic;

pub fn routes(router: Router) -> Router {
    // Just a simple hello world for now.
    router
        .route("/api/robot/", get(status))
        .route("/api/robot/register", post(register))
}

async fn status(Extension(service): Extension<Robotic>) -> Json<RobotStatus> {
    Json(service.robot().status())
}

async fn register(
    Extension(service): Extension<Robotic>,
    Json(robot_register): Json<RobotRegister>,
) -> Json<RobotConfig> {
    info!("A robot called {} registered", robot_register.name);
    info!(
        "With current networking: {:?}",
        robot_register.network_interfaces
    );
    service.robot().ping();
    Json(RobotConfig {})
}

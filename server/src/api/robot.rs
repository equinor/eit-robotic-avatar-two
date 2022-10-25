use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use common::{RobotConfig, RobotRegister, RobotStatus};
use log::{info, warn};

use crate::Robotic;

pub fn routes(router: Router) -> Router {
    // Just a simple hello world for now.
    router
        .route("/api/robot", get(status))
        .route("/api/robot/register", post(register))
        .route("/api/robot/token", get(get_token))
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
    service.robot().register(robot_register);
    Json(RobotConfig {})
}

async fn get_token(Extension(service): Extension<Robotic>) -> String {
    match service.auth().gen_token_for_robot() {
        Ok(token) => token,
        Err(err) => {
            warn!("/api/robot/token: {}", err);
            String::new()
        }
    }
}

use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use brain::{RobotRegister, RobotStatus};
use log::{info, warn};

use crate::Service;

pub fn routes(router: Router) -> Router {
    // Just a simple hello world for now.
    router
        .route("/api/robot", get(status))
        .route("/api/robot/register", post(register))
        .route("/api/robot/token", get(get_token))
        .route("/api/robot/pin", get(get_pin))
}

async fn status(Extension(service): Extension<Service>) -> Json<RobotStatus> {
    Json(service.robot().status())
}

async fn register(
    Extension(service): Extension<Service>,
    Json(robot_register): Json<RobotRegister>,
) {
    info!("A robot called {} registered", robot_register.name);
    info!(
        "With current networking: {:?}",
        robot_register.network_interfaces
    );
    service.robot().register(robot_register);
}

async fn get_token(Extension(service): Extension<Service>) -> String {
    match service.auth().gen_token_for_robot() {
        Ok(token) => token,
        Err(err) => {
            warn!("/api/robot/token: {}", err);
            String::new()
        }
    }
}

async fn get_pin(Extension(service): Extension<Service>) -> String {
    service.auth().issue_pin()
}

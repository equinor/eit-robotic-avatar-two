use axum::{
    routing::{get, post},
    Json, Router,
};
use common::{RobotConfig, RobotRegister};
use log::info;

pub fn routes(router: Router) -> Router {
    // Just a simple hello world for now.
    router
        .route("/api/robot/", get(|| async { "Hello, World!" }))
        .route("/api/robot/register", post(register))
}

async fn register(Json(robot_register): Json<RobotRegister>) -> Json<RobotConfig> {
    info!("A robot called {} registered", robot_register.name);
    info!(
        "With current networking: {:?}",
        robot_register.network_interfaces
    );
    Json(RobotConfig {})
}

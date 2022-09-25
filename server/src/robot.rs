use anyhow::Result;
use axum::{routing::{get, post}, Router, Json};
use common::{RobotConfig, RobotRegister};
use log::info;

pub async fn setup() -> Result<Router> {
    // Just a simple hello world for now.
    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/register", post(register));

    Ok(router)
}

async fn register(Json(robot_register): Json<RobotRegister>) -> Json<RobotConfig> {
    info!("A robot called {} registered", robot_register.name);

    Json(RobotConfig{})
}
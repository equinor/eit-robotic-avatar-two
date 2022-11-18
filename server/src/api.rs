mod auth;
mod files;
mod messaging;
mod minion;
mod robot;

use anyhow::Result;
use axum::{Extension, Router};
use log::debug;

use crate::{Config, Service};

pub async fn serve(config: Config, robotic: Service) -> Result<()> {
    debug!("Setting up routes");
    let api = Router::new();
    let api = messaging::routes(api);
    let api = robot::routes(api);
    let api = minion::routes(api);
    let api = auth::routes(api);
    let api = api.layer(Extension(robotic));
    let api = files::routes(api);

    debug!("Starting the server");
    println!("Binding to {} using http", config.bind_address);
    Ok(axum::Server::bind(&config.bind_address)
        .serve(api.into_make_service())
        .await?)
}

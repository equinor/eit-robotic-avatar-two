use axum::Router;
use anyhow::Result;

use crate::Config;

pub async fn serve(app: Router, config: &Config) -> Result<()> {
    println!("Binding to {} using http", config.bind_address);
    Ok(axum::Server::bind(&config.bind_address)
        .serve(app.into_make_service())
        .await?)
}

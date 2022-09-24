use anyhow::Result;
use axum::{routing::get, Router};

pub async fn setup() -> Result<Router> {
    // Just a simple hello world for now.
    let router = Router::new().route("/", get(|| async { "Hello, World!" }));

    Ok(router)
}

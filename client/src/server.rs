use common::{RobotStatus, SendMessage};
use gloo_net::http::Request;

#[derive(Clone)]
pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub async fn post_message(&self, msg: &SendMessage) {
        Request::post("/api/messaging")
            .json(&msg)
            .unwrap()
            .send()
            .await
            .unwrap();
    }

    pub async fn get_auth_login(&self) -> String {
        Request::get("/api/auth/login")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    pub async fn get_robot(&self) -> RobotStatus {
        Request::get("/api/robot")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

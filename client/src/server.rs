use common::{RobotStatus, SendMessage};
use gloo_net::http::Request;

#[derive(Clone)]
pub struct Server {
    header: String,
}

impl Server {
    pub fn new(token: &str) -> Server {
        Server {
            header: format!("Bearer {}", token),
        }
    }

    pub async fn get_auth_login() -> String {
        Request::get("/api/auth/login")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    pub async fn post_message(&self, msg: &SendMessage) {
        self.post("/api/messaging")
            .header("Authorization", &self.header)
            .json(&msg)
            .unwrap()
            .send()
            .await
            .unwrap();
    }

    pub async fn get_robot(&self) -> RobotStatus {
        self.get("/api/robot")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn get_robot_token(&self) -> String {
        self.get("/api/robot/token")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    fn get(&self, url: &str) -> Request {
        Request::get(url).header("Authorization", &self.header)
    }

    fn post(&self, url: &str) -> Request {
        Request::post(url).header("Authorization", &self.header)
    }
}

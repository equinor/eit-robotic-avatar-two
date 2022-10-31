use common::{RobotStatus, RtcMessage, SendMessage, Tracking};
use gloo_net::http::Request;
use gloo_timers::future::TimeoutFuture;

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

    pub async fn post_auth_pin(pin: String) -> String {
        Request::post("/api/auth/pin")
            .body(pin)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    pub async fn post_message(&self, msg: &SendMessage) {
        self.post("/api/messaging")
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

    pub async fn get_robot_pin(&self) -> String {
        self.get("/api/robot/pin")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    pub async fn post_minion_post_offers(&self, offers: &RtcMessage) {
        self.post_rtc("/api/minion/post_offer", offers).await;
    }
    pub async fn get_minion_pull_offers(&self) -> RtcMessage {
        self.pull_rtc("/api/minion/get_offer").await
    }
    pub async fn post_minion_post_answer(&self, answer: &RtcMessage) {
        self.post_rtc("/api/minion/post_answer", answer).await;
    }
    pub async fn get_minion_pull_answer(&self) -> RtcMessage {
        self.pull_rtc("/api/minion/get_answer").await
    }

    pub async fn post_minion_tracking(&self, tracking: &Tracking) {
        self.post("/api/minion/tracking")
            .json(&tracking)
            .unwrap()
            .send()
            .await
            .unwrap();
    }

    async fn post_rtc(&self, path: &str, payload: &RtcMessage) {
        self.post(path).json(payload).unwrap().send().await.unwrap();
    }

    async fn pull_rtc(&self, path: &str) -> RtcMessage {
        loop {
            let maybe = self.get(path).send().await.unwrap().json().await;

            if maybe.is_ok() {
                break maybe.unwrap();
            }

            TimeoutFuture::new(5000).await
        }
    }

    fn get(&self, url: &str) -> Request {
        Request::get(url).header("Authorization", &self.header)
    }

    fn post(&self, url: &str) -> Request {
        Request::post(url).header("Authorization", &self.header)
    }
}

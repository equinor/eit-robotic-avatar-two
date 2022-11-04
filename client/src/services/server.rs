use std::sync::Mutex;

use common::{RobotStatus, RtcMessage, SendMessage, Tracking};
use gloo_net::http::Request;
use gloo_timers::future::TimeoutFuture;

static HEADER: Mutex<String> = Mutex::new(String::new());

pub fn set_token(token: &str) {
    *HEADER.lock().unwrap() = format!("Bearer {}", &token);
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

pub async fn post_message(msg: &SendMessage) {
    post("/api/messaging")
        .json(&msg)
        .unwrap()
        .send()
        .await
        .unwrap();
}

pub async fn get_robot() -> RobotStatus {
    get("/api/robot")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn get_robot_token() -> String {
    get("/api/robot/token")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

pub async fn get_robot_pin() -> String {
    get("/api/robot/pin")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

pub async fn post_minion_post_offers(offers: &RtcMessage) {
    post_rtc("/api/minion/post_offer", offers).await;
}
pub async fn get_minion_pull_offers() -> RtcMessage {
    pull_rtc("/api/minion/get_offer").await
}
pub async fn post_minion_post_answer(answer: &RtcMessage) {
    post_rtc("/api/minion/post_answer", answer).await;
}
pub async fn get_minion_pull_answer() -> RtcMessage {
    pull_rtc("/api/minion/get_answer").await
}

pub async fn post_minion_tracking(tracking: &Tracking) {
    post("/api/minion/tracking")
        .json(&tracking)
        .unwrap()
        .send()
        .await
        .unwrap();
}

async fn post_rtc(path: &str, payload: &RtcMessage) {
    post(path).json(payload).unwrap().send().await.unwrap();
}

async fn pull_rtc(path: &str) -> RtcMessage {
    loop {
        let maybe = get(path).send().await.unwrap().json().await;

        if maybe.is_ok() {
            break maybe.unwrap();
        }

        TimeoutFuture::new(5000).await
    }
}

fn get(url: &str) -> Request {
    apply_auth(Request::get(url))
}

fn post(url: &str) -> Request {
    apply_auth(Request::post(url))
}

fn apply_auth(req: Request) -> Request {
    let header = HEADER.lock().unwrap();
    if header.is_empty() {
        req
    } else {
        req.header("Authorization", &header)
    }
}

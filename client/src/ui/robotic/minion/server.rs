use common::RtcMessage;
use gloo_net::http::Request;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::prelude::*;

pub async fn post_offers(offers: &RtcMessage) {
    post("/api/minion/post_offer", offers).await;
}
pub async fn pull_offers() -> RtcMessage {
    pull("/api/minion/get_offer").await
}
pub async fn post_answer(answer: &RtcMessage) {
    post("/api/minion/post_answer", answer).await;
}
pub async fn pull_answer() -> RtcMessage {
    pull("/api/minion/get_answer").await
}
pub async fn post_tracking(tracking: Tracking) {
    postTracking(tracking).await;
}

async fn post(path: &str, payload: &RtcMessage) {
    Request::post(path)
        .json(payload)
        .unwrap()
        .send()
        .await
        .unwrap();
}

async fn pull(path: &str) -> RtcMessage {
    loop {
        let maybe = Request::get(path).send().await.unwrap().json().await;

        if maybe.is_ok() {
            break maybe.unwrap();
        }

        TimeoutFuture::new(5000).await
    }
}

#[wasm_bindgen]
pub struct Tracking {
    pub head: Head,
    pub drive: Drive,
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone, Copy)]
pub struct Head {
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone, Copy)]
pub struct Drive {
    pub speed: f64,
    pub turn: f64,
}

#[wasm_bindgen(raw_module = "/js/modules/server.mjs")]
extern "C" {
    async fn postTracking(tracking: Tracking);
}

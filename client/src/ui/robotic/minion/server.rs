use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub async fn post_offers(offers: JsValue) {
    postOffers(offers).await;
}
pub async fn pull_offers() -> JsValue {
    pullOffers().await
}
pub async fn post_answer(answer: JsValue) {
    postAnswer(answer).await;
}
pub async fn pull_answer() -> JsValue {
    pullAnswer().await
}
pub async fn post_tracking(tracking: Tracking) {
    postTracking(tracking).await;
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
    async fn postOffers(offers: JsValue);
    async fn pullOffers() -> JsValue;
    async fn postAnswer(answer: JsValue);
    async fn pullAnswer() -> JsValue;
    async fn postTracking(tracking: Tracking);
}

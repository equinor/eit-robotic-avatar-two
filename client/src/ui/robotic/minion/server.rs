use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

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
    pub async fn postOffers(offers: JsValue);
    pub async fn pullOffers() -> JsValue;
    pub async fn postAnswer(answer: JsValue);
    pub async fn pullAnswer() -> JsValue;
    pub async fn postTracking(tracking: Tracking);
}

use js_sys::{Object, Reflect};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{RtcSdpType, RtcSessionDescription, RtcSessionDescriptionInit};

pub async fn post_offers(offers: (RtcSessionDescription, RtcSessionDescription)) {
    let value = Object::new();
    Reflect::set(&value, &"left".into(), &offers.0.into()).unwrap();
    Reflect::set(&value, &"right".into(), &offers.1.into()).unwrap();

    postOffers(value.into()).await;
}
pub async fn pull_offers() -> (RtcSessionDescriptionInit, RtcSessionDescriptionInit) {
    let value = pullOffers().await;
    let left = Reflect::get(&value, &"left".into()).unwrap();
    let left = into_rtc_init(&left);
    let right = Reflect::get(&value, &"right".into()).unwrap();
    let right = into_rtc_init(&right);

    (left, right)
}
pub async fn post_answer(answer: (RtcSessionDescription, RtcSessionDescription)) {
    let value = Object::new();
    Reflect::set(&value, &"left".into(), &answer.0.into()).unwrap();
    Reflect::set(&value, &"right".into(), &answer.1.into()).unwrap();

    postAnswer(value.into()).await;
}
pub async fn pull_answer() -> (RtcSessionDescriptionInit, RtcSessionDescriptionInit) {
    let value = pullAnswer().await;
    let left = Reflect::get(&value, &"left".into()).unwrap();
    let left = into_rtc_init(&left);
    let right = Reflect::get(&value, &"right".into()).unwrap();
    let right = into_rtc_init(&right);

    (left, right)
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

fn into_rtc_init(value: &JsValue) -> RtcSessionDescriptionInit {
    let type_ = Reflect::get(value, &"type".into()).unwrap();
    let type_ = RtcSdpType::from_js_value(&type_).unwrap();
    let spd = Reflect::get(value, &"sdp".into())
        .unwrap()
        .as_string()
        .unwrap();
    let mut init = RtcSessionDescriptionInit::new(type_);
    init.sdp(&spd);
    init
}

#[wasm_bindgen(raw_module = "/js/modules/server.mjs")]
extern "C" {
    async fn postOffers(offers: JsValue);
    async fn pullOffers() -> JsValue;
    async fn postAnswer(answer: JsValue);
    async fn pullAnswer() -> JsValue;
    async fn postTracking(tracking: Tracking);
}

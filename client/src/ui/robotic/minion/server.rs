use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(raw_module = "/js/modules/server.mjs")]
extern "C" {
    pub async fn postOffers(offers: JsValue);
    pub async fn pullAnswer() -> JsValue;
}

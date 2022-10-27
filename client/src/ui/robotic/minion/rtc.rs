use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::MediaStream;

pub async fn from_streams(streams: (MediaStream, MediaStream)) -> Connection {
    fromStreams(streams.0, streams.1).await.dyn_into().unwrap()
}

#[wasm_bindgen(raw_module = "/js/modules/rtc.mjs")]
extern "C" {
    pub type Connection;

    async fn fromStreams(left: MediaStream, right: MediaStream) -> JsValue;
}

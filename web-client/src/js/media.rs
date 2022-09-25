use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::MediaStream;
use weblog::console_warn;
use yew::Callback;

pub fn get_user_video(callback: Callback<MediaStream>) {
    spawn_local(async move {
        match user_video_id("test id").await.dyn_into() {
            Ok(media) => callback.emit(media),
            Err(err) => console_warn!("Fail to get user video", err),
        }
    });
}

#[wasm_bindgen(module = "/js/media_selector.js")]
extern "C" {
    async fn user_video_id(id: &str) -> JsValue;
}

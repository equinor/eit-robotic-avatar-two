use common::SendMessage;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

pub fn send_message(msg: &SendMessage, callback: Callback<()>) {
    let msg = msg.clone();
    spawn_local(async move {
        Request::post("/api/messaging")
            .json(&msg)
            .unwrap()
            .send()
            .await
            .unwrap();
        callback.emit(());
    });
}

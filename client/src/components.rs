use wasm_bindgen_futures::spawn_local;
use web_sys::MediaStream;
use yew::prelude::*;

use crate::services::{server, webrtc};

#[function_component(GenPin)]
pub fn gen_pin() -> Html {
    let pin = use_state(|| None);

    let onclick = {
        let pin = pin.clone();
        Callback::from(move |_| {
            let pin = pin.clone();
            spawn_local(async move { pin.set(Some(server::get_robot_pin().await)) })
        })
    };

    match &*pin {
        Some(s) => html! {<span class={"pin"}>{s}</span>},
        None => html! {<button {onclick}>{"Generate login Pin"}</button>},
    }
}

#[derive(Properties, PartialEq)]
pub struct HeadsetStreamProps {
    pub callback: Callback<(MediaStream, MediaStream)>,
}

#[function_component(HeadsetStream)]
pub fn headset_stream(props: &HeadsetStreamProps) -> Html {
    let started = use_state(|| false);

    let onclick = {
        let started = started.clone();
        let callback = props.callback.clone();
        Callback::from(move |_| {
            started.set(true);
            let callback = callback.clone();
            spawn_local(async move {
                callback.emit(webrtc::receive().await);
            })
        })
    };

    html! {<button disabled={*started} {onclick}>{"Start video link with minion robot"}</button>}
}

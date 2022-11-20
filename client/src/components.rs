use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::services::server;

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

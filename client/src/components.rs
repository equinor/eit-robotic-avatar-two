use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, MediaStream};
use yew::prelude::*;

use crate::services::{media, server, webrtc};

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
pub struct StreamProps {
    pub callback: Callback<(MediaStream, MediaStream)>,
}

#[function_component(HeadsetStream)]
pub fn headset_stream(props: &StreamProps) -> Html {
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

#[function_component(MinionStream)]
pub fn minion_stream(props: &StreamProps) -> Html {
    let started = use_state(|| false);

    let onclick = {
        let started = started.clone();
        let callback = props.callback.clone();
        Callback::from(move |_| {
            started.set(true);
            let callback = callback.clone();
            spawn_local(async move {
                let cam_id: (String, String) =
                    LocalStorage::get("minion_cam_id").unwrap_or_default();
                let left = media::get_user_video(&cam_id.0).await;
                let right = media::get_user_video(&cam_id.1).await;
                callback.emit((left.clone(), right.clone()));
                webrtc::send_video((left, right)).await;
            })
        })
    };

    html! {<button disabled={*started} {onclick}>{"Minion start sending."}</button>}
}

#[function_component(MediaSelect)]
pub fn media_select() -> Html {
    let cam_id: (String, String) = LocalStorage::get("minion_cam_id").unwrap_or_default();

    let left_id_change = Callback::from(|e: Event| {
        let mut cam_id: (String, String) = LocalStorage::get("minion_cam_id").unwrap_or_default();
        cam_id.0 = e
            .target()
            .expect("Event should have a target when dispatched")
            .unchecked_into::<HtmlInputElement>()
            .value();
        LocalStorage::set("minion_cam_id", cam_id).unwrap();
    });

    let right_id_change = Callback::from(|e: Event| {
        let mut cam_id: (String, String) = LocalStorage::get("minion_cam_id").unwrap_or_default();
        cam_id.1 = e
            .target()
            .expect("Event should have a target when dispatched")
            .unchecked_into::<HtmlInputElement>()
            .value();
        LocalStorage::set("minion_cam_id", cam_id).unwrap();
    });

    html! {
        <p>
            {"Left Camera ID:"} <input size={64} value={cam_id.0.clone()} onchange={left_id_change} /><br/>
            {"Right Camera ID:"} <input size={64} value={cam_id.1.clone()} onchange={right_id_change} />
        </p>
    }
}

#[function_component(DeviceList)]
pub fn device_list() -> Html {
    let devices = use_state(Vec::new);
    let first = use_mut_ref(|| true);
    if *first.borrow() {
        *first.borrow_mut() = false;
        let devices = devices.clone();
        spawn_local(async move {
            devices.set(media::list_video().await);
        })
    }

    let devices = devices.iter().map(|device_info| {
        html! {
            <li>{device_info.label()}{": "}{device_info.device_id()}</li>
        }
    });

    html! {
        <ul>
            {for devices}
        </ul>
    }
}

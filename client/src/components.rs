use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, JsValue,
};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, HtmlInputElement, HtmlVideoElement, MediaStream};
use yew::prelude::*;

use crate::services::{media, server, tracking::Track, webrtc};

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

#[derive(PartialEq, Eq, Properties, Clone)]
pub struct ViewportProps {
    pub streams: Option<(MediaStream, MediaStream)>,
}

#[function_component(Viewport)]
pub fn viewport(props: &ViewportProps) -> Html {
    let streams = match &props.streams {
        Some(s) => (Some(s.0.clone()), Some(s.1.clone())),
        None => (None, None),
    };

    let track = use_ref(Track::default);
    let canvas_ref = use_node_ref();
    let left_ref = use_node_ref();
    let right_ref = use_node_ref();
    let first_render = use_mut_ref(|| true);

    {
        let canvas_ref = canvas_ref.clone();
        let left_ref = left_ref.clone();
        let right_ref = right_ref.clone();
        use_effect(move || {
            let left = left_ref.cast().unwrap();
            let right = right_ref.cast().unwrap();

            if *first_render.borrow() {
                *first_render.borrow_mut() = false;
                let track = track.clone();
                let closure = Closure::new(move |value| track.send(value));
                setup_3d(canvas_ref.cast().unwrap(), &left, &right, &closure);
                closure.forget();
            }

            left.set_src_object(streams.0.as_ref());
            right.set_src_object(streams.1.as_ref());
            || ()
        });
    }

    html! {
        <div class={"viewport"}>
            <canvas ref={canvas_ref} />
            <video autoplay={true} ref={left_ref} />
            <video autoplay={true} ref={right_ref} />
        </div>
    }
}

#[wasm_bindgen(raw_module = "/js/viewport.mjs")]
extern "C" {
    fn setup_3d(
        canvas: HtmlCanvasElement,
        left: &HtmlVideoElement,
        right: &HtmlVideoElement,
        onTrack: &Closure<dyn FnMut(JsValue)>,
    );
}

mod cameras;
mod rtc;
mod viewport;

use std::{cell::RefCell, rc::Rc};

use gloo_storage::{LocalStorage, Storage};
use js_sys::Reflect;
use stylist::css;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlInputElement, MediaStream};
use weblog::console_log;
use yew::prelude::*;

use self::cameras::{list_devices, load_cams};
use self::rtc::{from_streams, Connection};
use self::viewport::Viewport;

#[derive(PartialEq, Eq, Properties)]
pub struct Props;

pub enum Msg {
    LeftCamId(String),
    RightCamId(String),
    Devices(Vec<(String, String)>),
    Source,
    Receiver,
    Cams((MediaStream, MediaStream)),
    Tracking(JsValue),
}

pub struct Minion {
    cam_id: (String, String),
    devices: Vec<(String, String)>,
    started: bool,
    cams: (Option<MediaStream>, Option<MediaStream>),
    sending: Rc<RefCell<bool>>,
}

impl Component for Minion {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let cam_id = LocalStorage::get("minion_cam_id").unwrap_or_default();

        let callback = ctx.link().callback(Msg::Devices);
        spawn_local(async move {
            let devices = list_devices().await;
            callback.emit(devices)
        });

        Minion {
            cam_id,
            devices: Vec::new(),
            started: false,
            cams: (None, None),
            sending: Rc::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();
        match msg {
            Msg::LeftCamId(id) => {
                self.cam_id.0 = id;
                LocalStorage::set("minion_cam_id", self.cam_id.clone()).unwrap();
            }
            Msg::RightCamId(id) => {
                self.cam_id.1 = id;
                LocalStorage::set("minion_cam_id", self.cam_id.clone()).unwrap();
            }
            Msg::Devices(devices) => self.devices = devices,
            Msg::Source => {
                self.started = true;

                let cam_id = self.cam_id.clone();
                let callback = link.callback(Msg::Cams);
                start_source(callback, cam_id);
            }
            Msg::Receiver => {
                self.started = true;

                let callback = link.callback(Msg::Cams);
                spawn_local(async move {
                    let streams = receiver().await;
                    let left = Reflect::get(&streams, &JsValue::from_str("left"))
                        .unwrap()
                        .dyn_into()
                        .unwrap();
                    let right = Reflect::get(&streams, &JsValue::from_str("right"))
                        .unwrap()
                        .dyn_into()
                        .unwrap();
                    callback.emit((left, right));
                });
            }
            Msg::Cams(cams) => self.cams = (Some(cams.0), Some(cams.1)),
            Msg::Tracking(value) => {
                let mut sending = self.sending.borrow_mut();
                if !(*sending) {
                    *sending = true;
                    drop(sending);

                    let sending = self.sending.clone();
                    spawn_local(async move {
                        tracking(value).await;
                        *sending.borrow_mut() = false;
                    });
                }
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let css = css!(
            r#"
            height: 100%;
            display: grid;
            box-sizing: border-box;
            grid-template-columns: 1fr;
            grid-template-rows: auto 1fr;
            grid-template-areas: 
                "ui"
                "view";
            gap: 16px 16px;
            padding: 8px;

            & > .ui {
                grid-area: ui;
            }
            
            & > .view {
                grid-area: view;
            }
        "#
        );

        let left_id_change = link.callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            Msg::LeftCamId(target.unchecked_into::<HtmlInputElement>().value())
        });

        let right_id_change = link.callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            Msg::RightCamId(target.unchecked_into::<HtmlInputElement>().value())
        });

        let devices = self.devices.iter().map(|(a, b)| {
            html! {
                <li>{a}{": "}{b}</li>
            }
        });

        html! {
            <div class={css}>
                <div class={"ui"}>
                    <h1>{"Robotic Avatar Demo"}</h1>
                    <p>
                        {"Left Camera ID:"} <input size={64} value={self.cam_id.0.clone()} onchange={left_id_change} /><br/>
                        {"Right Camera ID:"} <input size={64} value={self.cam_id.1.clone()} onchange={right_id_change} />
                        <ul>
                            {for devices}
                        </ul>
                    </p>
                    <p>
                        <button disabled={self.started} onclick={link.callback(|_| Msg::Source)}>{"Start as source"}</button>
                        <button disabled={self.started} onclick={link.callback(|_| Msg::Receiver)}>{"Start as receiver"}</button>
                    </p>
                </div>
                <Viewport class={"view"} left={self.cams.0.clone()} right={self.cams.1.clone()} on_track={link.callback(Msg::Tracking)}></Viewport>
            </div>
        }
    }
}

fn start_source(callback: Callback<(MediaStream, MediaStream)>, cam_id: (String, String)) {
    spawn_local(async move {
        let streams = load_cams(&cam_id.0, &cam_id.1).await;
        callback.emit(streams.clone());
        let con = from_streams(streams).await;
        let offers = con.createOffers().await;
        console_log!(&offers);
        source(con, offers).await
    });
}

#[wasm_bindgen(raw_module = "/js/view/RoboticAvatar.mjs")]
extern "C" {
    async fn source(con: Connection, offers: JsValue);
    async fn receiver() -> JsValue;
    async fn tracking(track: JsValue);
}

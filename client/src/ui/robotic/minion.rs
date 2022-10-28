mod cameras;
mod viewport;

use std::{cell::RefCell, rc::Rc};

use common::{Drive, Head, Tracking};
use stylist::css;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlInputElement, MediaStream};
use yew::prelude::*;

use crate::robotic::{MinionAction, MinionState};
use crate::services::{Server, WebRtc};

use self::cameras::{list_devices, load_cams};
use self::viewport::{Viewport, ViewportTracking};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub state: MinionState,
    pub actions: Callback<MinionAction>,
}

pub enum Msg {
    Devices(Vec<(String, String)>),
    Source,
    Receiver,
    Cams((MediaStream, MediaStream)),
    Tracking(ViewportTracking),
}

pub struct Minion {
    devices: Vec<(String, String)>,
    started: bool,
    cams: (Option<MediaStream>, Option<MediaStream>),
    sending: Rc<RefCell<bool>>,
    server: Server,
    webrtc: WebRtc,
}

impl Component for Minion {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::Devices);
        spawn_local(async move {
            let devices = list_devices().await;
            callback.emit(devices)
        });

        let server = Server::new("");

        Minion {
            devices: Vec::new(),
            started: false,
            cams: (None, None),
            sending: Rc::default(),
            server: server.clone(),
            webrtc: WebRtc::new(server),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();
        let state = &ctx.props().state;
        match msg {
            Msg::Devices(devices) => self.devices = devices,
            Msg::Source => {
                self.started = true;

                let cam_id = state.cam_id.clone();
                let callback = link.callback(Msg::Cams);
                start_source(callback, cam_id, self.webrtc.clone());
            }
            Msg::Receiver => {
                self.started = true;

                let callback = link.callback(Msg::Cams);
                start_receiver(callback, self.webrtc.clone());
            }
            Msg::Cams(cams) => self.cams = (Some(cams.0), Some(cams.1)),
            Msg::Tracking(value) => {
                let mut sending = self.sending.borrow_mut();
                if !(*sending) {
                    *sending = true;
                    drop(sending);

                    let sending = self.sending.clone();
                    let server = self.server.clone();
                    spawn_local(async move {
                        let tracking = Tracking {
                            head: Head {
                                rx: value.rx,
                                ry: value.ry,
                                rz: value.rz,
                            },
                            drive: Drive {
                                speed: value.l.y,
                                turn: value.l.x,
                            },
                        };
                        server.post_minion_tracking(&tracking).await;
                        *sending.borrow_mut() = false;
                    });
                }
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let state = &ctx.props().state;
        let actions = &ctx.props().actions;

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

        let left_id_change = actions.reform(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            MinionAction::LeftCamChange(target.unchecked_into::<HtmlInputElement>().value())
        });

        let right_id_change = actions.reform(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            MinionAction::RightCamChange(target.unchecked_into::<HtmlInputElement>().value())
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
                        {"Left Camera ID:"} <input size={64} value={state.cam_id.0.clone()} onchange={left_id_change} /><br/>
                        {"Right Camera ID:"} <input size={64} value={state.cam_id.1.clone()} onchange={right_id_change} />
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

fn start_source(
    callback: Callback<(MediaStream, MediaStream)>,
    cam_id: (String, String),
    webrtc: WebRtc,
) {
    spawn_local(async move {
        let streams = load_cams(&cam_id.0, &cam_id.1).await;
        callback.emit(streams.clone());
        webrtc.send_video(streams).await;
    });
}

fn start_receiver(callback: Callback<(MediaStream, MediaStream)>, webrtc: WebRtc) {
    spawn_local(async move {
        callback.emit(webrtc.receive().await);
    });
}

mod viewport;

use std::{cell::RefCell, rc::Rc};

use common::{Drive, Head, Tracking};
use stylist::css;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::robotic::{MinionAction, MinionState};
use crate::services::Server;

use self::viewport::{Viewport, ViewportTracking};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub state: MinionState,
    pub actions: Callback<MinionAction>,
}

pub enum Msg {
    Tracking(ViewportTracking),
}

pub struct Minion {
    sending: Rc<RefCell<bool>>,
    server: Server,
}

impl Component for Minion {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let server = Server::new("");

        Minion {
            sending: Rc::default(),
            server,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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

        let devices = state.devices.iter().map(|device_info| {
            html! {
                <li>{device_info.label()}{": "}{device_info.device_id()}</li>
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
                        <button disabled={state.started} onclick={actions.reform(|_| MinionAction::StartSending)}>{"Start sending video."}</button>
                        <button disabled={state.started} onclick={actions.reform(|_| MinionAction::StartReceiving)}>{"Start receiving video"}</button>
                    </p>
                </div>
                <Viewport class={"view"} left={state.streams.0.clone()} right={state.streams.1.clone()} on_track={link.callback(Msg::Tracking)}></Viewport>
            </div>
        }
    }
}

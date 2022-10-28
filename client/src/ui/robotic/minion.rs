mod viewport;

use stylist::css;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::robotic::{MinionAction, MinionState};

use self::viewport::Viewport;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub state: MinionState,
    pub actions: Callback<MinionAction>,
}

#[function_component(Minion)]
pub fn minion(props: &Props) -> Html {
    let state = &props.state;
    let actions = &props.actions;

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
            <Viewport class={"view"} left={state.streams.0.clone()} right={state.streams.1.clone()} on_track={actions.reform(MinionAction::Tracking)}></Viewport>
        </div>
    }
}

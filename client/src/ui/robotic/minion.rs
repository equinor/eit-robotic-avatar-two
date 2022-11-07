mod viewport;

use stylist::css;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_agent::use_bridge;

use crate::agents::{
    minion::{MinionAction, MinionState},
    MinionAgent,
};

use self::viewport::Viewport;

#[function_component(Minion)]
pub fn minion() -> Html {
    let state = use_state(MinionState::default);
    let agent = {
        let state = state.clone();
        use_bridge::<MinionAgent, _>(move |s| state.set(s))
    };
    let actions = Callback::from(move |a| agent.send(a));

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

mod viewport;

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, MediaStream};
use yew::prelude::*;
use yew_agent::use_bridge;

use crate::agents::{
    minion::{MinionAction, MinionState},
    MinionAgent,
};

use self::viewport::Viewport;

#[derive(Properties, PartialEq, Eq)]
pub struct MinionProps {
    pub left: Option<MediaStream>,
    pub right: Option<MediaStream>,
}

#[function_component(Minion)]
pub fn minion(props: &MinionProps) -> Html {
    let state = use_state(MinionState::default);
    let agent = {
        let state = state.clone();
        use_bridge::<MinionAgent, _>(move |s| state.set(s))
    };
    let actions = Callback::from(move |a| agent.send(a));

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
        <div class={"minion"}>
            <Viewport left={props.left.clone()} right={props.right.clone()} on_track={actions.reform(MinionAction::Tracking)}></Viewport>
            <div class={"ui"}>
                <p>
                    {"Left Camera ID:"} <input size={64} value={state.cam_id.0.clone()} onchange={left_id_change} /><br/>
                    {"Right Camera ID:"} <input size={64} value={state.cam_id.1.clone()} onchange={right_id_change} />
                    <ul>
                        {for devices}
                    </ul>
                </p>
            </div>
        </div>
    }
}

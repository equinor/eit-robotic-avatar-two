mod viewport;

use web_sys::MediaStream;
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

    let devices = state.devices.iter().map(|device_info| {
        html! {
            <li>{device_info.label()}{": "}{device_info.device_id()}</li>
        }
    });

    html! {
        <div class={"minion"}>
            <Viewport left={props.left.clone()} right={props.right.clone()} on_track={actions.reform(MinionAction::Tracking)}></Viewport>
            <div class={"ui"}>
                <ul>
                    {for devices}
                </ul>
            </div>
        </div>
    }
}

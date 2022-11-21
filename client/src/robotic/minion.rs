mod viewport;

use web_sys::MediaStream;
use yew::prelude::*;
use yew_agent::use_bridge;

use crate::agents::{minion::MinionAction, MinionAgent};

use self::viewport::Viewport;

#[derive(Properties, PartialEq, Eq)]
pub struct MinionProps {
    pub left: Option<MediaStream>,
    pub right: Option<MediaStream>,
}

#[function_component(Minion)]
pub fn minion(props: &MinionProps) -> Html {
    let agent = use_bridge::<MinionAgent, _>(|_| ());
    let actions = Callback::from(move |a| agent.send(a));

    html! {
        <div class={"minion"}>
            <Viewport left={props.left.clone()} right={props.right.clone()} on_track={actions.reform(MinionAction::Tracking)}></Viewport>
        </div>
    }
}

mod viewport;

use web_sys::MediaStream;
use yew::prelude::*;

use self::viewport::Viewport;

#[derive(Properties, PartialEq, Eq)]
pub struct MinionProps {
    pub left: Option<MediaStream>,
    pub right: Option<MediaStream>,
}

#[function_component(Minion)]
pub fn minion(props: &MinionProps) -> Html {
    html! {
        <div class={"minion"}>
            <Viewport left={props.left.clone()} right={props.right.clone()}></Viewport>
        </div>
    }
}

use crate::robotic::RobotState;
use time::{ext::NumericalDuration, OffsetDateTime};
use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct Props {
    pub state: RobotState,
}

#[function_component(Robot)]
pub fn robot(props: &Props) -> Html {
    let online = props.state.last_seen.map_or_else(
        || "Never".to_string(),
        |t| {
            let now = OffsetDateTime::now_utc();
            let time_since = now - t;
            let time_since = time_since.whole_seconds().seconds();
            format!("{} ago", time_since)
        },
    );

    html! {
        <div>
            <h1>{"Robot the tester."}</h1>
            <h2>{"Last Seen Online: "}{online}</h2>
        </div>
    }
}

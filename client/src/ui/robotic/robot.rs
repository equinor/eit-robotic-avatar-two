use crate::robotic::{RobotState, RoboticMsg};
use common::Interface;
use time::{ext::NumericalDuration, OffsetDateTime};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub state: RobotState,
    pub actions: Callback<RoboticMsg>,
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

    let interfaces = props.state.interfaces.iter().map(interface_to_row);

    html! {
        <div>
            <h1>{"Robot the tester."}</h1>
            <h2>{"Last Seen Online: "}{online}</h2>

            <table>
                <thead>
                    <tr>
                        <th>{"Interface name"}</th>
                        <th>{"IP Address"}</th>
                        <th>{"Broadcast"}</th>
                        <th>{"Netmask"}</th>
                        <th>{"Mac Address"}</th>
                    </tr>
                </thead>
                <tbody>
                    {interfaces.collect::<Html>()}
                </tbody>
            </table>

            {gen_token(props.actions.clone(), props.state.token.as_ref())}
        </div>
    }
}

fn interface_to_row(interface: &Interface) -> Html {
    html! {
        <tr>
            <td>{&interface.name}</td>
            <td>{&interface.ip}</td>
            <td>{&interface.broadcast}</td>
            <td>{&interface.netmask}</td>
            <td>{&interface.mac}</td>
        </tr>
    }
}

fn gen_token(actions: Callback<RoboticMsg>, token: Option<&String>) -> Html {
    let token = token.map(|s| &**s).unwrap_or("");

    html! {
        <div>
            <button onclick={move |_| actions.emit(RoboticMsg::GenRobotToken) }>{"Generate token for Robot"}</button>
            <pre>{token}</pre>
        </div>
    }
}

use common::Interface;
use time::{ext::NumericalDuration, OffsetDateTime};
use yew::prelude::*;
use yew_agent::use_bridge;

use crate::robotic::{robot::{RobotState, RobotAction}, RobotAgent};

#[function_component(Robot)]
pub fn robot() -> Html {
    let state = use_state(RobotState::default);
    let agent = {
        let state = state.clone();
        use_bridge::<RobotAgent, _>(move |s| state.set(s))
    };
    let actions = Callback::from(move |msg| agent.send(msg));

    let online = state.last_seen.map_or_else(
        || "Never".to_string(),
        |t| {
            let now = OffsetDateTime::now_utc();
            let time_since = now - t;
            let time_since = time_since.whole_seconds().seconds();
            format!("{} ago", time_since)
        },
    );

    let interfaces = state.interfaces.iter().map(interface_to_row);

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

            {gen_token(actions.clone(), state.token.as_ref())}
            {gen_pin(actions.clone(), state.pin.as_ref())}
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

fn gen_token(actions: Callback<RobotAction>, token: Option<&String>) -> Html {
    let token = token.map(|s| &**s).unwrap_or("");

    html! {
        <div>
            <button onclick={move |_| actions.emit(RobotAction::GenToken) }>{"Generate token for Robot"}</button>
            <pre>{token}</pre>
        </div>
    }
}

fn gen_pin(actions: Callback<RobotAction>, pin: Option<&String>) -> Html {
    let pin = pin.map(|s| &**s).unwrap_or("");

    html! {
        <div>
            <button onclick={move |_| actions.emit(RobotAction::GenPin) }>{"Generate login Pin"}</button>
            <pre>{pin}</pre>
        </div>
    }
}

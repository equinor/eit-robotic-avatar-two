use common::Interface;
use time::{ext::NumericalDuration, OffsetDateTime};
use yew::prelude::*;
use yew_agent::use_bridge;

use crate::agents::{robot::RobotState, RobotAgent};

#[function_component(Robot)]
pub fn robot() -> Html {
    let state = use_state(RobotState::default);
    let _ = {
        let state = state.clone();
        use_bridge::<RobotAgent, _>(move |s| state.set(s))
    };

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

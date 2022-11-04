use web_sys::MediaDeviceKind;
use yew::prelude::*;
use yew_agent::use_bridge;

use crate::agents::{
    media::{MediaActions, MediaState},
    MediaAgent,
};

#[function_component(MediaSelector)]
pub fn media_selector() -> Html {
    let state = use_state(MediaState::default);
    let agent = {
        let state = state.clone();
        use_bridge::<MediaAgent, _>(move |s| state.set(s))
    };
    use_ref(|| agent.send(MediaActions::GetMedia));

    fn kind_name(kind: MediaDeviceKind) -> &'static str {
        match kind {
            MediaDeviceKind::Audioinput => "Microphone",
            MediaDeviceKind::Audiooutput => "Speaker",
            MediaDeviceKind::Videoinput => "Camera",
            _ => "Unknown",
        }
    }

    let devices = state.devices.iter().map(|info| {
        html!(
            <tr>
                <td>{kind_name(info.kind())}</td>
                <td>{info.label()}</td>
            </tr>
        )
    });

    html! {
        <div>
            <h2> {"Media Selector"}</h2>
            <table>
                <thead>
                    <tr>
                        <th>{"Type"}</th>
                        <th>{"Name"}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td></td>
                        <td>{"None"}</td>
                    </tr>
                    {for devices}
                </tbody>
            </table>
        </div>
    }
}

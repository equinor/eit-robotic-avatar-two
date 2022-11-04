use web_sys::MediaDeviceKind;
use yew::{html, Component, Context, Html, Properties};
use yew_agent::{Bridge, Bridged};

use crate::agents::{
    media::{MediaActions, MediaState},
    MediaAgent,
};

#[derive(Properties, PartialEq, Eq)]
pub struct Props {}

pub enum Msg {
    State(MediaState),
}

pub struct MediaSelector {
    _agent: Box<dyn Bridge<MediaAgent>>,
    state: MediaState,
}

impl Component for MediaSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut agent = MediaAgent::bridge(ctx.link().callback(Msg::State));
        agent.send(MediaActions::GetMedia);

        MediaSelector {
            _agent: agent,
            state: MediaState::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::State(state) => {
                self.state = state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        fn kind_name(kind: MediaDeviceKind) -> &'static str {
            match kind {
                MediaDeviceKind::Audioinput => "Microphone",
                MediaDeviceKind::Audiooutput => "Speaker",
                MediaDeviceKind::Videoinput => "Camera",
                _ => "Unknown",
            }
        }

        let devices = self.state.devices.iter().map(|info| {
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
}

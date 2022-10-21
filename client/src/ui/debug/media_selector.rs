use web_sys::MediaDeviceKind;
use yew::{html, Callback, Component, Context, Html, Properties};

use crate::robotic::{MediaState, RoboticMsg};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub media: MediaState,
    pub actions: Callback<RoboticMsg>,
}

pub struct MediaSelector {}

impl Component for MediaSelector {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        props.actions.emit(RoboticMsg::Media);

        MediaSelector {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        fn kind_name(kind: MediaDeviceKind) -> &'static str {
            match kind {
                MediaDeviceKind::Audioinput => "Microphone",
                MediaDeviceKind::Audiooutput => "Speaker",
                MediaDeviceKind::Videoinput => "Camera",
                _ => "Unknown",
            }
        }

        let devices = props.media.devices.iter().map(|info| {
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

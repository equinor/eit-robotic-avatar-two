use web_sys::{MediaDeviceInfo, MediaDeviceKind, MediaStream};
use yew::{html, Component, Context, Html, Properties};

use crate::robotic::MediaService;

#[derive(PartialEq, Eq, Properties)]
pub struct Props{
    pub media: MediaService
}

pub struct MediaSelector {
    device_list: Vec<MediaDeviceInfo>,
}

pub enum Msg {
    LeftVideo(MediaStream),
    DeviceInfo(Vec<MediaDeviceInfo>),
}

impl Component for MediaSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        props.media.get_user_media_callback(ctx.link().callback(Msg::LeftVideo));
        MediaSelector {
            device_list: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            Msg::LeftVideo(_video) => {
                props.media
                    .enumerate_devices_callback(ctx.link().callback(Msg::DeviceInfo));
            }
            Msg::DeviceInfo(list) => {
                self.device_list = list;
            }
        }
        true
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

        let devices = self.device_list.iter().map(|info| {
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

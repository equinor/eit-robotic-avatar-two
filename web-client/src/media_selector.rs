use web_sys::{MediaStream, MediaDeviceInfo};
use weblog::console_log;
use yew::{html, Component, Context, Html, Properties};

use crate::js::media::{get_user_video, enumerate_devices_callback};

#[derive(PartialEq, Eq, Properties)]
pub struct Props;

pub struct MediaSelector {}

pub enum Msg {
    LeftVideo(MediaStream),
    DeviceInfo(Vec<MediaDeviceInfo>)
}

impl Component for MediaSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        get_user_video(ctx.link().callback(Msg::LeftVideo));
        MediaSelector {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            "Media Selector"
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LeftVideo(video) => {
                console_log!(video);
                enumerate_devices_callback(ctx.link().callback(Msg::DeviceInfo));
            },
            Msg::DeviceInfo(list) => {
                for device in list.iter() {
                    console_log!(device);
                }
            }
        }
        false
    }
}

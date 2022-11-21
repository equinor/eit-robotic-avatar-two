use web_sys::MediaStream;
use yew::prelude::*;

use crate::components::{
    DeviceList, GenPin, GenToken, HeadsetStream, MediaSelect, MinionStatus, MinionStream, Viewport,
};

#[derive(PartialEq, Eq, Properties)]
pub struct Props {}

pub enum Msg {
    SetStreams((MediaStream, MediaStream)),
    ToggleAdvanced,
}

pub struct Robotic {
    streams: Option<(MediaStream, MediaStream)>,
    show_advanced: bool,
}

impl Component for Robotic {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Robotic {
            streams: None,
            show_advanced: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetStreams(s) => self.streams = Some(s),
            Msg::ToggleAdvanced => self.show_advanced = !self.show_advanced,
        };
        true
    }

    #[allow(clippy::let_unit_value)]
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class={"robotic"}>
                <h1>{"Robotic Avatar"}</h1>

                <ol>
                    <li>{"Generate pin for Meta Quest 2 headset: "} <GenPin/> </li>
                    <li><HeadsetStream callback={link.callback(Msg::SetStreams)}/></li>
                    <li>{"When you see the video click the ENTER VR button at the bottom of the viewport"}</li>
                </ol>

                <Viewport streams={self.streams.clone()}/>

                <h2 onclick={link.callback(|_| Msg::ToggleAdvanced)}>{"Advanced and minion settings."}</h2>

                if self.show_advanced {
                    <MediaSelect/>
                    <DeviceList/>
                    <MinionStream callback={link.callback(Msg::SetStreams)}/>

                    <MinionStatus/>
                    <GenToken/>
                }
            </div>
        }
    }
}

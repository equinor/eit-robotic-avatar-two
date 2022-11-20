mod minion;
mod robot;

pub use self::minion::Minion;
pub use self::robot::Robot;

use web_sys::MediaStream;
use yew::prelude::*;

use crate::components::{GenPin, HeadsetStream};

#[derive(PartialEq, Eq, Properties)]
pub struct Props {}

pub enum Msg {
    SetStreams((MediaStream, MediaStream)),
}

pub struct Robotic {
    streams: Option<(MediaStream, MediaStream)>,
}

impl Component for Robotic {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Robotic { streams: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetStreams(s) => self.streams = Some(s),
        };
        true
    }

    #[allow(clippy::let_unit_value)]
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class={"robotic"}>
                <h1 class="header">{"Robotic Avatar"}</h1>
                <content>
                    <ol>
                        <li>{"Generate pin for Meta Quest 2 headset: "} <GenPin/> </li>
                        <li><HeadsetStream callback={link.callback(Msg::SetStreams)}/></li>
                        <li>{"When you see the video click the ENTER VR button at the bottom of the screen"}</li>
                    </ol>

                    {
                        match &self.streams {
                            Some(streams) => html!(<Minion left={streams.0.clone()} right={streams.1.clone()}/>),
                            None =>  html!(<Minion/>)
                        }
                    }

                    <Robot/>
                </content>
            </div>
        }
    }
}

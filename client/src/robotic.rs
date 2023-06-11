use web_sys::MediaStream;
use yew::prelude::*;

use crate::{
    components::{
        DeviceList, GenPin, GenToken, HeadsetStream, MediaSelect, MinionStatus, MinionStream,
    },
    headset::{headset, Wrapper},
};

#[derive(PartialEq, Eq, Properties)]
pub struct Props {}

pub enum Msg {
    SetStreams((MediaStream, MediaStream)),
    ToggleAdvanced,
}

pub struct Robotic {
    show_advanced: bool,
    headset: Wrapper,
    headset_ref: NodeRef,
}

impl Component for Robotic {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Robotic {
            show_advanced: false,
            headset: Wrapper::new(),
            headset_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetStreams(s) => self.headset.set_streams(&Some(s)),
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

                <div ref={self.headset_ref.clone()}/>

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

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let root = self.headset_ref.cast().unwrap();
            headset(&root, &self.headset);
        }
    }
}

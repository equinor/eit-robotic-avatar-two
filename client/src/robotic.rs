mod minion;
mod robot;

pub use self::minion::Minion;
pub use self::robot::Robot;

use yew::prelude::*;

use crate::components::GenPin;

#[derive(PartialEq, Eq, Properties)]
pub struct Props {}

pub struct Robotic {}

impl Component for Robotic {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Robotic {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    #[allow(clippy::let_unit_value)]
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={"robotic"}>
                <h1 class="header">{"Robotic Avatar"}</h1>
                <content>
                    <ol>
                        <li>{"Generate pin for Meta Quest 2 headset: "} <GenPin/> </li>
                        <li>{"Start video link with minion robot"}</li>
                        <li>{"When you see the video click the ENTER VR button at the bottom of the screen"}</li>
                    </ol>



                    <Robot/>
                    <Minion/>
                </content>
            </div>
        }
    }
}

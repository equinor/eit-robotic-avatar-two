mod minion;
mod robot;

pub use self::minion::Minion;
pub use self::robot::Robot;

use yew::prelude::*;

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
                    <Robot/>
                    <Minion/>
                </content>
            </div>
        }
    }
}

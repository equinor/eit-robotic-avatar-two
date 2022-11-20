mod minion;
mod robot;

pub use self::minion::Minion;
pub use self::robot::Robot;

use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct Props {}

#[derive(PartialEq, Eq)]
pub enum Msg {
    Robot,
    Minion,
}

pub struct Robotic {
    page: Msg,
}

impl Component for Robotic {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Robotic { page: Msg::Robot }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if self.page != msg {
            self.page = msg;
            true
        } else {
            false
        }
    }

    #[allow(clippy::let_unit_value)]
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let content = match self.page {
            Msg::Robot => {
                html!(<Robot/>)
            }
            Msg::Minion => {
                html!(<Minion/>)
            }
        };

        html! {
            <div class={"robotic"}>
                <h1 class="header">{"Robotic Avatar"}</h1>
                <menu>
                    <button onclick={link.callback(|_| Msg::Robot)}>
                        <p class="icon">{"🤖"}</p>
                        <p>{"Robot"}</p>
                    </button>
                    <button onclick={link.callback(|_| Msg::Minion)}>
                        <img src="img/minion.jpg"/>
                        <p>{"Minion"}</p>
                    </button>
                    <button disabled={true}>
                        <p>{"Rocky"}</p>
                    </button>
                </menu>
                <content>{content}</content>
            </div>
        }
    }
}

mod minion;
mod robot;

pub use self::minion::Minion;
pub use self::robot::Robot;

use stylist::css;
use yew::prelude::*;

use crate::RoboticState as Model;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: Classes,
    pub model: Model,
}

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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();

        let css = css!(
            r#"
            display: grid;
            grid-template-columns: min-content 1fr;
            grid-template-rows: min-content 1fr;
            grid-template-areas: 
                "header header"
                "menu robot";
            
            & > h1 {
                grid-area: header;
                background-color: lightblue;
                border-bottom: black solid 1px;
                margin: 0;
                padding: 0.2em;
            }

            & > menu {
                grid-area: menu;
                border-right: black solid 1px;
                margin: 0;
                padding: 0.2em;
                width: 15em;
            }

            & > menu > button{
                width: 100%;
                margin: 1em 0;
            }

            & > menu > button > img{
                width: 100%;
                object-fit: contain;
            }

            & > menu > button > .icon{
                font-size: 10em;
                margin: 0;
            }

            & > content {
                grid-area: robot;
                margin: 0;
                padding: 0.2em;
            }
        "#
        );

        let class = classes!(props.class.clone(), css);

        let content = match self.page {
            Msg::Robot => html!(<Robot state={props.model.robot.clone()}/>),
            Msg::Minion => html!(<Minion/>),
        };

        html! {
            <div class={class}>
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

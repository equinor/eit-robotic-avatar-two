use stylist::css;
use yew::prelude::*;

use crate::Robotic as Model;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: Classes,
    pub model: Model,
}

pub enum Msg {}

pub struct Robotic;

impl Component for Robotic {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Robotic
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let css = css!(
            r#"
            display: grid;
            grid-template-columns: min-content 1fr;
            grid-template-rows: min-content 1fr;
            grid-template-areas: 
                "header header"
                "menu robot";
        "#
        );

        let class = classes!(props.class.clone(), css);

        let header_css = css!(
            r#"
            grid-area: header;
            background-color: lightblue;
            border-bottom: black solid 1px;
            margin: 0;
            padding: 0.2em;
        "#
        );

        let menu_css = css!(
            r#"
            grid-area: menu;
            border-right: black solid 1px;
            margin: 0;
            padding: 0.2em;
            min-width: 10em;
        "#
        );

        let robot_css = css!(
            r#"
            grid-area: robot;
            margin: 0;
            padding: 0.2em;
        "#
        );

        html! {
            <div class={class}>
                <h1 class={header_css}>{"Robotic Avatar"}</h1>
                <div class={menu_css}>{"Future robot menu"}</div>
                <div class={robot_css}>{"Future robot UI"}</div>
            </div>
        }
    }
}

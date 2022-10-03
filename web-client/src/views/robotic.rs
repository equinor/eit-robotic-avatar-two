use stylist::css;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: Classes,
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

        html! {
            <div class={class}>
                <h1 class={css!("grid-area: header;")}>{"Robotic Avatar"}</h1>
                <div class={css!("grid-area: menu;")}>{"Future robot menu"}</div>
                <div class={css!("grid-area: robot;")}>{"Future robot UI"}</div>
            </div>
        }
    }
}

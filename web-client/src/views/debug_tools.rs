use stylist::css;
use yew::prelude::*;

use super::media_selector::MediaSelector;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: Classes,
}

pub enum Msg {}

pub struct DebugTools;

impl Component for DebugTools {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        DebugTools
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let css = css!(
            r#"
            display: grid;
            grid-template-rows: min-content min-content;
            grid-template-areas: 
                "menu"
                "tool";
        "#
        );

        let class = classes!(props.class.clone(), css);

        html! {
            <div class={class}>
                <div class={css!("grid-area: menu;")}>{"Debug tools: "}</div>
                <div class={css!("grid-area: tool;")}><MediaSelector /></div>
            </div>
        }
    }
}

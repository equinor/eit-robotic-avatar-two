use stylist::css;
use yew::prelude::*;

use super::{MediaSelector, MessagingDebug};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: Classes,
}

#[derive(PartialEq, Eq)]
pub enum Tab {
    None,
    Media,
    Message,
}

pub struct DebugTools {
    current_tab: Tab,
}

impl Component for DebugTools {
    type Message = Tab;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        DebugTools {
            current_tab: Tab::None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if self.current_tab == msg {
            self.current_tab = Tab::None
        } else {
            self.current_tab = msg;
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();

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

        let tab = match self.current_tab {
            Tab::None => html!(),
            Tab::Media => html!(<MediaSelector />),
            Tab::Message => html!(<MessagingDebug />),
        };

        html! {
            <div class={class}>
                <div class={css!("grid-area: menu;")}>
                    {"Debug tools: "}
                    <button onclick={link.callback(|_| Tab::Media)}>{"Media Debugger"}</button>
                    <button onclick={link.callback(|_| Tab::Message)}>{"Messaging Debugger"}</button>
                    <button onclick={link.callback(|_| Tab::None)}>{"X"}</button>
                </div>
                <div class={css!("grid-area: tool;")}>{tab}</div>
            </div>
        }
    }
}

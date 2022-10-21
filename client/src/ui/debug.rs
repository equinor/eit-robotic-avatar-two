mod media_selector;
mod messaging_debug;

use stylist::css;
use yew::prelude::*;

use crate::robotic::RoboticMsg;
use crate::RoboticState;

pub use self::media_selector::MediaSelector;
pub use self::messaging_debug::MessagingDebug;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: Classes,
    pub state: RoboticState,
    pub actions: Callback<RoboticMsg>,
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
            Tab::Media => {
                html!(<MediaSelector media={props.state.media.clone()} actions={props.actions.clone()}/>)
            }
            Tab::Message => html!(<MessagingDebug />),
        };

        let header_css = css!(
            r#"
            grid-area: menu;
            background-color: lightgray;
            border: black solid 1px;
            border-radius: 10px 10px 0 0;
            padding: 5px;

            & > * {
                margin: 5px;
            }
        "#
        );

        let box_css = css!(
            r#"
            grid-area: tool;
            border-left: black solid 1px;
            border-right: black solid 1px;
        "#
        );

        html! {
            <div class={class}>
                <div class={header_css}>
                    {"Debug tools: "}
                    <button onclick={link.callback(|_| Tab::Media)}>{"Media Debugger"}</button>
                    <button onclick={link.callback(|_| Tab::Message)}>{"Messaging Debugger"}</button>
                    <button onclick={link.callback(|_| Tab::None)}>{"X"}</button>
                </div>
                <div class={box_css}>{tab}</div>
            </div>
        }
    }
}

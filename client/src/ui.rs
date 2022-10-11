mod debug;
mod login;
mod robotic;

pub use debug::DebugTools;
pub use login::Login;
pub use robotic::Robotic;

use crate::Robotic as Model;

use stylist::{css, yew::Global};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub robotic: Model,
}

pub struct Ui {
    page: Page,
}

#[derive(PartialEq, Eq)]
pub enum Page {
    Login,
    Main,
}

impl Component for Ui {
    type Message = Page;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Ui { page: Page::Login }
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

        let global_css = css!(
            r#"
                html, body, #robotic-avatar{
                    height: 100%;
                    width: 100%;
                    margin: 0;
                }

                #robotic-avatar {
                    display: grid;
                    grid-template-rows: 1fr min-content;
                    grid-template-areas: 
                        "main"
                        "debug"
                }
            "#
        );

        let page = match self.page {
            Page::Login => {
                html!(<Login class={css!("grid-area: main;")} on_login={link.callback(|_| Page::Main)} model={props.robotic.clone()}/>)
            },
            Page::Main => {
                html! {
                    <>
                        <Robotic class={css!("grid-area: main;")} model={props.robotic.clone()}/>
                        <DebugTools class={css!("grid-area: debug;")} model={props.robotic.clone()}/>
                    </>
                }
            }
        };

        html! {
            <>
                <Global css={global_css} />
                {page}
            </>
        }
    }
}

mod debug;
mod login;
mod robotic;

pub use debug::DebugTools;
pub use login::Login;
pub use robotic::Robotic;

use crate::robotic::{Robotic as App, RoboticMsg};

use stylist::{css, yew::Global};
use yew::prelude::*;

pub struct Ui {
    page: Page,
    robotic: App,
}

pub enum Msg {
    State,
    Page(Page),
    Action(RoboticMsg),
}

#[derive(PartialEq, Eq)]
pub enum Page {
    Login,
    Main,
}

impl Component for Ui {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        let robotic = App::new(link.callback(|_| Msg::State));

        Ui {
            page: Page::Login,
            robotic,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::State => true,
            Msg::Page(page) => {
                if self.page != page {
                    self.page = page;
                    true
                } else {
                    false
                }
            }
            Msg::Action(action) => {
                self.robotic.action(action);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = self.robotic.state();
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
                html!(<Login class={css!("grid-area: main;")} on_login={link.callback(|_| Msg::Page(Page::Main))}/>)
            }
            Page::Main => {
                html! {
                    <>
                        <Robotic class={css!("grid-area: main;")} model={state.clone()}/>
                        <DebugTools class={css!("grid-area: debug;")} state={state} actions={link.callback(Msg::Action)}/>
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

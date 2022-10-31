mod debug;
mod login;
mod robotic;

pub use debug::DebugTools;
pub use login::Login;
pub use robotic::Robotic;

use crate::{
    robotic::{Robotic as App, RoboticMsg},
    services::Server,
};

use stylist::{css, yew::Global};
use yew::prelude::*;

pub struct Ui {
    robotic: Option<App>,
}

pub enum Msg {
    State,
    Action(RoboticMsg),
    Login(Server),
}

impl Component for Ui {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Ui { robotic: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();
        match msg {
            Msg::State => true,
            Msg::Action(action) => {
                if let Some(robotic) = &mut self.robotic {
                    robotic.action(action);
                    true
                } else {
                    false
                }
            }
            Msg::Login(server) => {
                let robotic = App::new(link.callback(|_| Msg::State), server);
                self.robotic = Some(robotic);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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

        let page = if let Some(robotic) = &self.robotic {
            let state = robotic.state();
            html! {
                <>
                    <Robotic class={css!("grid-area: main;")} model={state.clone()} actions={link.callback(Msg::Action)}/>
                    <DebugTools class={css!("grid-area: debug;")} state={state} actions={link.callback(Msg::Action)}/>
                </>
            }
        } else {
            html!(<Login class={css!("grid-area: main;")} on_login={link.callback(Msg::Login)}/>)
        };

        html! {
            <>
                <Global css={global_css} />
                {page}
            </>
        }
    }
}

mod debug;
mod login;
mod robotic;

pub use debug::DebugTools;
pub use login::Login;
pub use robotic::Robotic;

use stylist::{css, yew::Global};
use yew::prelude::*;

pub struct Ui {
    logged_in: bool,
}

pub enum Msg {
    Login,
}

impl Component for Ui {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Ui { logged_in: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Login => {
                self.logged_in = true;
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

        let page = if self.logged_in {
            html! {
                <>
                    <Robotic class={css!("grid-area: main;")}/>
                    <DebugTools class={css!("grid-area: debug;")}/>
                </>
            }
        } else {
            html!(<Login class={css!("grid-area: main;")} on_login={link.callback(|_|Msg::Login)}/>)
        };

        html! {
            <>
                <Global css={global_css} />
                {page}
            </>
        }
    }
}

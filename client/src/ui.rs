mod login;
mod robotic;

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
                html, body{
                    height: 100%;
                    width: 100%;
                    margin: 0;
                }
            "#
        );

        let page = if self.logged_in {
            html! (<Robotic/>)
        } else {
            html!(<Login on_login={link.callback(|_|Msg::Login)}/>)
        };

        html! {
            <>
                <Global css={global_css} />
                {page}
            </>
        }
    }
}

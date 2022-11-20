mod agents;
mod auth;
mod components;
mod login;
mod robotic;
mod services;

use yew::prelude::*;

use login::Login;
use robotic::Robotic;

pub struct App {
    logged_in: bool,
}

pub enum Msg {
    Login,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App { logged_in: false }
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

        if self.logged_in {
            html! (<Robotic/>)
        } else {
            html!(<Login on_login={link.callback(|_|Msg::Login)}/>)
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

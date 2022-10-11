use yew::prelude::*;

use crate::Robotic;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: Classes,
    pub model: Robotic,
    pub on_login: Callback<()>
}


pub struct Login;

impl Component for Login {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Login
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let _link = ctx.link();

        html! {
            <div class={props.class.clone()}>
                <button onclick={props.on_login.reform(|_|())}>{"Login by nothing"}</button>
            </div>
        }
    }
}
use stylist::css;
use yew::prelude::*;

use crate::Robotic;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: Classes,
    pub model: Robotic,
    pub on_login: Callback<()>,
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

        let css = css!(
            r#"
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 1.5em;

                & > div {
                    border: black solid 1px;
                    border-radius: 10px 10px 0 0;
                    overflow: hidden;
                }

                h1 {
                    background-color: lightblue;
                    border-bottom: black solid 1px;
                    margin: 0;
                    padding: 0.2em;
                }

                p {
                    margin: 0;
                    padding: 0.2em;
                }

                button {
                    font-size: 0.85em;
                }
            "#
        );

        let class = classes!(props.class.clone(), css);

        html! {
            <div class={class}>
                <div>
                    <h1>{"Robotic Avatar"}</h1>
                    <p>{"Welcome please select a login method:"}</p>
                    <p><button onclick={props.on_login.reform(|_|())}>{"Login by nothing"}</button></p>
                </div>
            </div>
        }
    }
}

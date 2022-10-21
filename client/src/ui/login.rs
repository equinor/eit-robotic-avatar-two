use stylist::css;
use yew::prelude::*;

use crate::{auth::Auth, server::Server};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: Classes,
    pub on_login: Callback<Server>,
}

pub enum Msg {
    Nothing,
    AzureAD,
}

pub struct Login {
    auth: Auth,
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        Login {
            auth: Auth::new(props.on_login.clone()),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Nothing => self.auth.nothing(),
            Msg::AzureAD => self.auth.start_azure_ad(),
        };
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();

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
                    <p><button onclick={link.callback(|_| Msg::Nothing)}>{"Login using nothing"}</button></p>
                    <p><button onclick={link.callback(|_| Msg::AzureAD)}>{"Login using AzureAD"}</button></p>
                </div>
            </div>
        }
    }
}

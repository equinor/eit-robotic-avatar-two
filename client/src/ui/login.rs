use stylist::css;
use wasm_bindgen_futures::spawn_local;
use weblog::console_error;
use yew::prelude::*;

use crate::{robotic::server, Robotic};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: Classes,
    pub model: Robotic,
    pub on_login: Callback<()>,
}

pub enum Msg {
    Done,
    AzureADStart,
}

pub struct Login;

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Login
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Done => ctx.props().on_login.emit(()),
            Msg::AzureADStart => spawn_local(azure_ad()),
        }
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
                    <p><button onclick={link.callback(|_| Msg::Done)}>{"Login using nothing"}</button></p>
                    <p><button onclick={link.callback(|_| Msg::AzureADStart)}>{"Login using AzureAD"}</button></p>
                </div>
            </div>
        }
    }
}

async fn azure_ad() {
    let url = server::auth_login().await;
    if !url.is_empty() {
        let window = web_sys::window().unwrap();
        let location = window.location();
        if let Err(err) = location.assign(&url) {
            console_error!("Location assign error: ", err);
        }
    }
}

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::auth::Auth;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub on_login: Callback<()>,
}

pub enum Msg {
    AzureAD,
    Pin(String),
    PinLogin,
}

pub struct Login {
    auth: Auth,
    pin: String,
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        Login {
            auth: Auth::new(props.on_login.clone()),
            pin: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AzureAD => {
                self.auth.start_azure_ad();
                false
            }
            Msg::Pin(pin) => {
                self.pin = pin;
                true
            }
            Msg::PinLogin => {
                self.auth.pin(self.pin.clone());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let pin_change = link.callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            Msg::Pin(target.unchecked_into::<HtmlInputElement>().value())
        });

        html! {
            <div class={"login"}>
                <div>
                    <h1>{"Robotic Avatar"}</h1>
                    <p>{"Welcome please select a login method:"}</p>
                    <p><button onclick={link.callback(|_| Msg::AzureAD)}>{"Login using AzureAD"}</button></p>
                    <p>
                        { "Pin:" }<input type="text" onchange={pin_change}/><br/>
                        <button onclick={link.callback(|_| Msg::PinLogin)}>{"Login using pin"}</button>
                    </p>
                </div>
            </div>
        }
    }
}

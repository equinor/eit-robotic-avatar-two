use common::SendMessage;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement};
use yew::{events::Event, html, Callback, Component, Context, Html, Properties};

use crate::robotic::RoboticMsg;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub actions: Callback<RoboticMsg>,
}

pub struct MessagingDebug {
    to_send: SendMessage,
}

pub enum Msg {
    Topic(String),
    Type(String),
    Payload(String),
    Send,
}

impl Component for MessagingDebug {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        MessagingDebug {
            to_send: SendMessage {
                topic: String::new(),
                msg_type: String::new(),
                payload: String::new(),
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            Msg::Topic(topic) => self.to_send.topic = topic,
            Msg::Type(msg_type) => self.to_send.msg_type = msg_type,
            Msg::Payload(payload) => self.to_send.payload = payload,
            Msg::Send => props
                .actions
                .emit(RoboticMsg::SendMessage(self.to_send.clone())),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let topic_change = link.callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            Msg::Topic(target.unchecked_into::<HtmlInputElement>().value())
        });

        let type_change = link.callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            Msg::Type(target.unchecked_into::<HtmlInputElement>().value())
        });

        let payload_change = link.callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            Msg::Payload(target.unchecked_into::<HtmlTextAreaElement>().value())
        });

        html! {
            <div>
                <h2> {"Messaging Debug"}</h2>
                <div>
                <h3> {"Send message"}</h3>
                {"Topic: "} <input type="text" onchange={topic_change}/> <br/>
                {"Type: "} <input type="text" onchange={type_change}/> <br/>
                {"Payload:"} <br/>
                <textarea onchange={payload_change}></textarea><br/>
                <button onclick={link.callback(|_| Msg::Send)}>{"Send"}</button>
                </div>
            </div>
        }
    }
}

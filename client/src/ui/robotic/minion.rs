use gloo_storage::{LocalStorage, Storage};
use stylist::css;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::{EventTarget, HtmlElement, HtmlInputElement};
use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct Props;

pub enum Msg {
    LeftCamId(String),
    RightCamId(String),
}

pub struct Minion {
    node_ref: NodeRef,
    root: JsValue,
    cam_id: (String, String),
}

impl Component for Minion {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let cam_id = LocalStorage::get("minion_cam_id").unwrap_or_default();

        Minion {
            node_ref: NodeRef::default(),
            root: JsValue::null(),
            cam_id,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LeftCamId(id) => {
                self.cam_id.0 = id;
                LocalStorage::set("minion_cam_id", self.cam_id.clone()).unwrap();
            }
            Msg::RightCamId(id) => {
                self.cam_id.1 = id;
                LocalStorage::set("minion_cam_id", self.cam_id.clone()).unwrap();
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let css = css!(
            r#"
            height: 100%;
            display: grid;
            box-sizing: border-box;
            grid-template-columns: 1fr;
            grid-template-rows: auto 1fr;
            grid-template-areas: 
                "ui"
                "view";
            gap: 16px 16px;
            padding: 8px;

            & > .ui {
                grid-area: ui;
            }
            
            & > .view {
                grid-area: view;
            }
        "#
        );

        let left_id_change = link.callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            Msg::LeftCamId(target.unchecked_into::<HtmlInputElement>().value())
        });

        let right_id_change = link.callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            Msg::RightCamId(target.unchecked_into::<HtmlInputElement>().value())
        });

        html! {
            <div class={css}>
                <div class={"ui"}>
                    <h1>{"Robotic Avatar Demo"}</h1>
                    <p>
                        {"Left Camera ID:"} <input size={64} value={self.cam_id.0.clone()} onchange={left_id_change} /><br/>
                        {"Right Camera ID:"} <input size={64} value={self.cam_id.1.clone()} onchange={right_id_change} />
                    </p>
                </div>
                <div class={"view"} ref={self.node_ref.clone()}>

                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.root = minion_root(self.node_ref.cast().unwrap());
        }
        render(&self.root, &self.cam_id.0, &self.cam_id.1)
    }
}

#[wasm_bindgen(raw_module = "/js/index.mjs")]
extern "C" {
    fn minion_root(root_elem: HtmlElement) -> JsValue;
    fn render(root: &JsValue, left_cam_id: &str, right_cam_id: &str);
}

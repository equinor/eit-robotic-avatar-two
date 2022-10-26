use stylist::css;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct Props;

pub struct Minion {
    node_ref: NodeRef,
    root: JsValue,
}

impl Component for Minion {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Minion {
            node_ref: NodeRef::default(),
            root: JsValue::null(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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

        html! {
            <div class={css}>
                <div class={"ui"}>
                    <h1>{"Robotic Avatar Demo"}</h1>
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
        let props = ReactProps {};
        render(&self.root, props)
    }
}

#[wasm_bindgen]
struct ReactProps {}

#[wasm_bindgen(raw_module = "/js/index.mjs")]
extern "C" {
    fn minion_root(root_elem: HtmlElement) -> JsValue;
    fn render(root: &JsValue, props: ReactProps);
}

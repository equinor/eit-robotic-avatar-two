use stylist::css;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct Props;

pub struct Minion {
    node_ref: NodeRef,
}

impl Component for Minion {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Minion {
            node_ref: NodeRef::default(),
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
        "#
        );

        html! {
            <div class={css} ref={self.node_ref.clone()}></div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            robotic_main(self.node_ref.cast().unwrap())
        }
    }
}

#[wasm_bindgen(raw_module = "/js/index.mjs")]
extern "C" {
    fn robotic_main(root_elem: HtmlElement);
}
